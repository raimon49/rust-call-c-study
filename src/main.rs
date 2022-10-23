extern crate libc;
mod raw;

fn main() {
    {
        use std::os::raw::c_char;
        use std::ffi::CString;
        use std::ffi::CStr;

        // Rustの実行ファイルとリンクされるCライブラリで定義されている関数を宣言
        extern {
            fn strlen(s: *const c_char) -> usize;
            static environ: *mut *mut c_char;
        }

        let rust_str = "I'll be back";
        let null_terminated = CString::new(rust_str).unwrap(); // NULL文字で終端されたCの文字列を作る（引数にNULLが含まれているとCの文字列は作成できずエラーとなるためunwrapで取り出す必要がある）

        // externブロック内で宣言された関数を呼ぶ時はunsafeとして扱われる
        unsafe {
            // as_ptr()は文字列の先頭を *const c_char で返すため strlen() に渡せる
            assert_eq!(strlen(null_terminated.as_ptr()), 12);

            if !environ.is_null() && !(*environ).is_null() {
                // environの最初の要素を借用したCStrを作る
                let var = CStr::from_ptr(*environ);
                println!("first environment variable: {}",
                         var.to_string_lossy()); // to_string_lossy()はCow<str>を返す
            }
        }
    }
    {
        use std::ffi::CString;
        use std::mem;
        use std::ptr;
        use std::ffi::CStr;
        use std::os::raw::c_int;
        use std::os::raw::c_char;

        fn check(activity: &'static str, status: c_int) -> c_int {
            if status < 0 {
                unsafe {
                    let error = &*raw::giterr_last();
                    println!("error while {}: {} ({})",
                             activity,
                             CStr::from_ptr(error.message).to_string_lossy(),
                             error.klass);
                    std::process::exit(1);
                }
            }

            status
        }

        unsafe fn show_commit(commit: *const raw::git_commit) {
            let author = raw::git_commit_author(commit);

            let name = CStr::from_ptr((*author).name).to_string_lossy();
            let email = CStr::from_ptr((*author).email).to_string_lossy();
            println!("{} <{}>\n", name, email);

            let message = raw::git_commit_message(commit);
            println!("{}", CStr::from_ptr(message).to_string_lossy());
        }

        let path = std::env::args().skip(1).next()
            .expect("usage: git-toy PATH");
        let path = CString::new(path)
            .expect("path contains null characters");

        unsafe {
            check("initializing library", raw::git_libgit2_init());

            // 指定されたパスのGitリポジトリをオープンし、結果をチェック
            // 引数の&mut repoは暗黙的にrawポインタへ型変換され*mut *mut git_repository型で渡される
            let mut repo = ptr::null_mut();
            check("opening repository",
                raw::git_repository_open(&mut repo, path.as_ptr()));

            let c_name = b"HEAD\0".as_ptr() as *const c_char;
            let mut oid = mem::uninitialized(); // まったく初期化されていない任意の型を返す
            check("looking up HEAD",
                raw::git_reference_name_to_id(&mut oid, repo, c_name));

            let mut commit = ptr::null_mut(); // mem::uninitialized()でもいいが危険なのでnullで初期化
            check("looking up commit",
                raw::git_commit_lookup(&mut commit, repo, &oid));

            // 変数commitに格納されたコミットを表示
            show_commit(commit);

            // commitオブジェクトの解放
            raw::git_commit_free(commit);

            // repositoryオブジェクトの解放
            raw::git_repository_free(repo);

            check("shutting down library",
                raw::git_libgit2_shutdown());
        }
    }
}
