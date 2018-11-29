local Any = {}
function Any:is() return true end

local String = {}
function String:is(s)
  return type(s) == "string"
end

local Number = {}
function Number:is(n)
  return type(n) == "number"
end

local Function = {}
function Function:is(f)
  return type(f) == "function"
end

local Boolean = {}
function Boolean:is(b)
  return type(b) == "boolean"
end

local Nil = {}
function Nil:is(n)
  return type(n) == "nil"
end

local List = function(ty)
  local obj = {}
  function obj:is(l)
    if type(l) ~= "table" then return false end
    for _, val in ipairs(l) do
      if not ty:is(val) then return false end
    end
    return true
  end
  return obj
end

local Table = function(kType, vType)
  local obj = {}
  function obj:is(t)
    if type(t) ~= "table" then return false end
    for key, val in pairs(t) do
      if not kType:is(key) then return false end
      if not vType:is(val) then return false end
    end
    return true
  end
  return obj
end

return {
  Any = Any,
  String = String,
  Number = Number,
  Function = Function,
  Boolean = Boolean,
  Nil = Nil,
  List = List,
  Table = Table,
}
