-- luacheck: std max+busted
require 'busted.runner'()

_G.NATIVE_POINT = true
local Point = require "point.point"

describe("Point", function()
  it("has a constructor", function()
    local p = Point:new(1, 2)
    assert.are.equals(p:x(), 1)
    assert.are.equals(p:y(), 2)
  end)

  it("can be updated", function()
    local p = Point:new(1, 2)
    p:set_x(2)
    p:set_y(3)
    assert.are.equals(p:x(), 2)
    assert.are.equals(p:y(), 3)
  end)

  it("can be added/subtracted", function()
    local p = Point:new(1, 2) + Point:new(3, 4) - Point:new(1, 1)
    assert.are.equals(p:x(), 3)
    assert.are.equals(p:y(), 5)
  end)

  it("dist working", function()
    local p1 = Point:new(0, 0)
    local p2 = Point:new(3, 4)
    assert.are.equals(p1:dist(p2), 5)
  end)

  it("print working", function()
    local p1 = Point:new(0, 0)
    local p2 = Point:new(3, 4)
    assert(tostring(p1).." "..tostring(p2) == "{0, 0} {3, 4}")
  end)
end)
