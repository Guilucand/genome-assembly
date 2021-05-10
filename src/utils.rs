use std::path::{Path, PathBuf};
use std::sync::atomic::{AtomicUsize, Ordering};
use std::thread::JoinHandle;
use std::time::Duration;

pub struct Utils;

static THREADS_COUNTER: AtomicUsize = AtomicUsize::new(0);

impl Utils {
    #[inline]
    pub fn pos_from_letter(letter: u8) -> u8 {
        match letter as char {
            'A' => 0,
            'C' => 1,
            'G' => 2,
            'T' => 3,
            'N' => 4,
            _ => panic!("Wrong letter {}", letter),
        }
    }

    pub fn get_bucket_index(bucket_file: impl AsRef<Path>) -> usize {
        let mut file_path = bucket_file.as_ref().to_path_buf();

        while let Some(extension) = file_path.extension() {
            if extension != "lz4" {
                if let Some(extension) = extension.to_str() {
                    match extension.parse() {
                        Ok(bucket_index) => return bucket_index,
                        Err(_) => {}
                    };
                }
            }
            file_path = file_path.with_extension("");
        }
        panic!(
            "Cannot find bucket index for file {:?}",
            bucket_file.as_ref()
        );
    }

    pub fn generate_bucket_names(
        root: impl AsRef<Path>,
        count: usize,
        suffix: Option<&str>,
    ) -> Vec<PathBuf> {
        (0..count)
            .map(|i| {
                root.as_ref().with_extension(format!(
                    "{}{}",
                    i,
                    match suffix {
                        None => String::from(""),
                        Some(s) => format!(".{}", s),
                    }
                ))
            })
            .collect()
    }

    pub fn thread_safespawn<F: FnOnce() + Send + 'static>(func: F) {
        THREADS_COUNTER.fetch_add(1, Ordering::Relaxed);
        std::thread::spawn(|| {
            func();
            THREADS_COUNTER.fetch_sub(1, Ordering::Relaxed);
        });
    }

    pub fn join_all() {
        while THREADS_COUNTER.load(Ordering::Relaxed) != 0 {
            std::thread::sleep(Duration::from_secs(1));
        }
    }
}

pub fn cast_static<T: ?Sized>(val: &T) -> &'static T {
    unsafe { std::mem::transmute(val) }
}

pub fn cast_static_mut<T: ?Sized>(val: &mut T) -> &'static mut T {
    unsafe { std::mem::transmute(val) }
}
