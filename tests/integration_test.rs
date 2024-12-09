use std::fs;
use std::path::Path;

use pypaste::process_string;

fn process_file(path: &Path) -> String {
    let display = path.display();
    let content =
        fs::read_to_string(path).unwrap_or_else(|err| panic!("couldn't read {}: {}", display, err));
    process_string(&content)
}

#[test]
fn test_process_python_file() {
    let path = Path::new("tests/resources/test.py");
    let processed_content = process_file(path);
    println!("Processed content:\n{}", processed_content); // Print content for debugging
    assert_eq!(processed_content,"class Foo:\n    def __init__(self, test):\n        self.test = test\n    def run(self):\n        print(self.test)\n\n\ndef test():\n    print(\"line1\")\n    print(\"line3\")\n\n\nfoo = Foo()\nfoo.run()\nif True:\n    print(\"True\")\nelif False:\n    print(\"False\")\nelse:\n    print(\"False\")\n", "Processed content not as expected");
}

#[test]
fn test_indents() {
    let path = Path::new("tests/resources/indents.py");
    let processed_content = process_file(path);
    println!("Processed content:\n{}", processed_content); // Print content for debugging
    assert_eq!(
        processed_content, "foo = 3\nprint(foo)\nbar = 5\n",
        "Fixed indent not processed properly"
    );
}
