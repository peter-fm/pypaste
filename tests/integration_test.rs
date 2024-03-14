use std::fs;
use std::path::Path;

use pypaste::process_string;

#[test]
fn test_process_python_file() {
    let path = Path::new("tests/resources/test.py");
    let display = path.display();

    let content =
        fs::read_to_string(path).unwrap_or_else(|err| panic!("couldn't read {}: {}", display, err));

    let processed_content = process_string(&content);

    assert!(
        processed_content == "class Foo:\n    def __init__(self, test):\n        self.test = test\n    def run(self):\n        print(self.test)\n\n\ndef test():\n    print('line1')\n    print('line3')\n\n\nfoo = Foo()\nfoo.run()\n",
        "Processed content not as expected"
    );
}
