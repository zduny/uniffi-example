import kotlin.test.assertEquals

import org.zduny.vectors.*

assertEquals(compareLength(Vector.scalar(5.0), Vector.vec2(3.0, 4.0)), 
    ComparisonResult.EQUAL)
