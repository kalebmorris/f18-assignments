local function guard(t, keys)
  setmetatable(t, {__newindex = function(t, k, v)
    local contains = false
    for _, key in ipairs(keys) do
      if k == key then
        contains = true
      end
    end
    if contains then
      rawset(t, k, v)
    else
      error("Invalid key " .. k)
    end
  end})
end

local t = {d = 0}
guard(t, {"a", "b"})
assert(pcall(function() t.a = 0 end))
assert(pcall(function() t.b = 0 end))
assert(not pcall(function() t.c = 0 end))
assert(pcall(function() t.d = 1 end))


local function multilink(t, parents)
  setmetatable(t, {__index = function(t, k)
    for _, parent in ipairs(parents) do
      if parent[k] ~= nil then
        return parent[k]
      end
    end
  end})
end

local t = {a = 0}
multilink(t, {{x = 1}, {x = 2, y = 2}, {z = 3}})
assert(t.a == 0)
assert(t.x == 1)
assert(t.y == 2)
assert(t.z == 3)
