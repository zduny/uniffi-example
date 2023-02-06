import unittest
import simple


class TestHello(unittest.TestCase):
    def test_hello(self):
        self.assertEqual(simple.hello(simple.Person("Daniel")),
                         "Hello Daniel!", "Should be `Hello Daniel!`")


class TestAdd(unittest.TestCase):
    def test_add(self):
        self.assertEqual(simple.add(2, 3), 5, "Should be 5")


if __name__ == '__main__':
    unittest.main()
