import kotlin.test.assertEquals

import org.zduny.simple.*
import org.zduny.vectors.*

assertEquals(hello(Pet("Tom")), "Hello Tom!")
assertEquals(Person("Daniel", Vector.vec2(0.0, 0.0)).getName(), "Daniel")
assertEquals(Person("Daniel", Vector.vec2(0.0, 0.0)).getLocation().norm(), 0.0)
assertEquals(add(2UL, 4UL), 6UL)
assertEquals(testEnumToString(TestEnum.A), "A")
