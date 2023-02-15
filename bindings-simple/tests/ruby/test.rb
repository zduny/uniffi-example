# frozen_string_literal: true

require 'test/unit'
require 'simple'

include Test::Unit::Assertions

assert_equal Simple.hello(Simple::Pet.new("Tom")), "Hello Tom!"
assert_equal Simple::Person.new("Daniel").get_name(), "Daniel"
assert_equal Simple.add(2, 4), 6
assert_equal Simple.test_enum_to_string(Simple::TestEnum::A), "A"
