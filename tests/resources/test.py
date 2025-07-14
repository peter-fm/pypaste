class Foo:
    def __init__(self, test):
        self.test = test

    def run(self):
        print(self.test)


def test():
    print("line1")

    print("line3")


foo = Foo("demo")


foo.run()


if True:
    print("True")


elif False:
    print("False")
else:
    print("False")
