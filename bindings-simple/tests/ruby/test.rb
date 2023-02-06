require "test/unit"
require "simple"
 
class TestAdd < Test::Unit::TestCase
  def test_hello
    assert_equal(Simple.hello(Simple::Person.new("Daniel")), "Hello Daniel!")
  end

  def test_add
    assert_equal(5, Simple.add(2, 3))
  end
end
