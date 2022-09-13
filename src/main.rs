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
        use std::os::raw::c_int;

        // libgit2-devパッケージを入れておく
        // 独自ビルドしたlibgit2を利用する場合は、Cargo.tomlと同じディレクトリにbuild.rsを容易し、
        // [packcage]
        // build = "build.rs"
        // をCargo.tomlに記述してbuildスクリプトのmain()を実効させる
        // ex) fn main() {
        //      println!(r"/cargo:rustc-link-search=native=/home/jimb/libgit2-0.25.1/build");
        // }
        #[link(name = "git2")]
        extern {
            pub fn git_libgit2_init() -> c_int;
            pub fn git_libgit2_shutdown() -> c_int;
        }

        unsafe {
            git_libgit2_init();
            git_libgit2_shutdown();
        }
    }
}
