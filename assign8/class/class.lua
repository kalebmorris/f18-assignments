local types = require "class.types"

local Object
Object = {
  new = function()
    local inst = {}
    inst.type = Object.type
    return inst
  end,

  constructor = function() end,

  isHelper = function(class, ty)
    if rawequal(class, ty) then return true end
    for _, parent in ipairs(ty.bases) do
      if Object.isHelper(class, parent) then return true end
    end
    return false
  end,

  is = function(class, val)
    return Object.isHelper(class, val:type())
  end,

  type = function(_) return Object end,

  datatypes = {},

  bases = {}
}

local function class(bases, methods, datatypes)
  setmetatable(datatypes, {__index = function(_, k)
    for _, parent in ipairs(bases) do
      if parent.datatypes[k] ~= nil then
        return parent.datatypes[k]
      end
    end
  end})

  local Class = {}
  Class.bases = bases
  Class.datatypes = datatypes
  Class.type = function(_) return Class end
  methods(Class)

  function Class:new(...)
    local inst = {}

    local mt = {
      __index = function(_, k)
        return Class[k]
      end,

      __newindex = function(t, k, v)
        if datatypes[k] ~= nil then
          if datatypes[k]:is(v) then
            rawset(t, k, v)
          else
            error("Value does not match the declared type - "..v)
          end
        else
          error("Field is not declared in datatypes - "..k)
        end
      end
    }

    for k, v in pairs(Class) do
      if string.sub(k, 1, 2) == "__" then
        mt[k] = v
      end
    end

    setmetatable(inst, mt)

    if Class.constructor then inst:constructor(...) end

    return inst
  end

  function Class:is(val)
    return Object.is(self, val)
  end

  setmetatable(Class, {
    __index = function(_, k)
      for _, parent in ipairs(bases) do
        if parent[k] ~= nil then
          return parent[k]
        end
      end
    end
  })

  return Class
end

return {
  Object = Object,
  class = class,
}
