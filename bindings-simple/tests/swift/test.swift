import simple

assert(hello(pet: Pet(name: "Tom")) == "Hello Tom!", "hello works")
assert((try? Person(name: "Daniel", location: Vector.vec2(x: 0, y: 0)))!.getName() == "Daniel", "getName works")
assert((try? Person(name: "Daniel", location: Vector.vec2(x: 0, y: 0)))!.getLocation().norm() == 0)
assert(add(a: 2, b: 4) == 6, "add works")
assert(testEnumToString(testEnum: TestEnum.a) == "A", "testEnumToString works")
