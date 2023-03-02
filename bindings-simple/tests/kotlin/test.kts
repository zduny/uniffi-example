import kotlin.test.assertEquals

import org.zduny.simple.*

assertEquals(hello(Pet("Tom")), "Hello Tom!")
assertEquals(Person("Daniel").getName(), "Daniel")
assertEquals(add(2UL, 4UL), 6UL)
assertEquals(testEnumToString(TestEnum.A), "A")
