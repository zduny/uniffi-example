import simple

assert(hello(pet: Pet(name: "Tom")) == "Hello Tom!", "hello works")
assert(Person(name: "Daniel").getName() == "Daniel", "getName works")
assert(add(a: 2, b: 4) == 6, "add works")
assert(testEnumToString(testEnum: TestEnum.a) == "A", "testEnumToString works")
