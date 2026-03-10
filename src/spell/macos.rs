#[cfg(target_os = "macos")]
mod imp {
    use std::ffi::CStr;
    use std::os::raw::c_char;

    use cocoa::base::{id, nil};
    use cocoa::foundation::{NSAutoreleasePool, NSRange, NSString, NSUInteger};
    use objc::{class, msg_send, sel, sel_impl};

    const NS_NOT_FOUND: NSUInteger = NSUInteger::MAX;

    pub fn first_suggestion(word: &str) -> Result<Option<String>, String> {
        if word.is_empty() {
            return Ok(None);
        }

        unsafe {
            let _pool = NSAutoreleasePool::new(nil);
            let checker: id = msg_send![class!(NSSpellChecker), sharedSpellChecker];
            if checker == nil {
                return Err(String::from("NSSpellChecker is unavailable"));
            }

            let ns_word = NSString::alloc(nil).init_str(word);
            let misspelled: NSRange =
                msg_send![checker, checkSpellingOfString: ns_word startingAt: 0usize];

            if misspelled.location == NS_NOT_FOUND {
                return Ok(None);
            }

            let guesses: id = msg_send![
                checker,
                guessesForWordRange: misspelled
                inString: ns_word
                language: nil
                inSpellDocumentWithTag: 0isize
            ];

            if guesses == nil {
                return Ok(None);
            }

            let count: NSUInteger = msg_send![guesses, count];
            if count == 0 {
                return Ok(None);
            }

            let first: id = msg_send![guesses, objectAtIndex: 0usize];
            Ok(Some(nsstring_to_string(first)))
        }
    }

    unsafe fn nsstring_to_string(value: id) -> String {
        let utf8: *const c_char = msg_send![value, UTF8String];
        if utf8.is_null() {
            return String::new();
        }

        CStr::from_ptr(utf8).to_string_lossy().into_owned()
    }
}

#[cfg(not(target_os = "macos"))]
mod imp {
    pub fn first_suggestion(_word: &str) -> Result<Option<String>, String> {
        Err(String::from(
            "the macOS spellchecker backend is only available on macOS",
        ))
    }
}

pub fn first_suggestion(word: &str) -> Result<Option<String>, String> {
    imp::first_suggestion(word)
}
