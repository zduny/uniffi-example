import unittest
import simple
import vectors

class TestHello(unittest.TestCase):
    def test_hello(self):
        self.assertEqual(simple.hello(simple.Pet("Tom")),
                         "Hello Tom!", "Should be `Hello Tom!`")

class TestGetName(unittest.TestCase):
    def test_get_name(self):
        self.assertEqual(simple.Person("Daniel").get_name(),
                         "Daniel", "Should be `Daniel`")

class TestAdd(unittest.TestCase):
    def test_add(self):
        self.assertEqual(simple.add(2, 4), 6, "Should be 6")
        
class TestEnumToString(unittest.TestCase):
    def test_enum_to_string(self):
        self.assertEqual(simple.test_enum_to_string(simple.TestEnum.A),
                         "A", "Should be `A`")

if __name__ == '__main__':
    unittest.main()
