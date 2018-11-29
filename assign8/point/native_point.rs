#![allow(dead_code, non_camel_case_types, non_snake_case, non_upper_case_globals)]

mod lauxlib;

use lauxlib::*;
use std::ffi::{CString};
use std::os::raw::{c_char, c_int};
use std::{mem, ptr};

fn cstr<T: Into<String>>(s: T) -> *const c_char {
  CString::new(s.into()).unwrap().into_raw()
}

static mut POINT_NATIVE: *const c_char = ptr::null();

unsafe fn lua_pushcfunction(L: *mut lua_State, f: lua_CFunction) {
	lua_pushcclosure(L, f, 0);
}

unsafe fn luaL_getmetatable(L: *mut lua_State, n: *const c_char) -> c_int {
  lua_getfield(L, LUA_REGISTRYINDEX, n)
}

struct Point {
  x: lua_Number,
  y: lua_Number
}

unsafe extern "C" fn point_new(L: *mut lua_State) -> c_int {
  let p = lua_newuserdata(L, mem::size_of::<Point>()) as *mut Point;
  let x = luaL_checknumber(L, -3);
  let y = luaL_checknumber(L, -2);
  (*p).x = x as lua_Number;
  (*p).y = y as lua_Number;
  luaL_getmetatable(L, POINT_NATIVE);
  lua_setmetatable(L, -2);
  return 1;
}

unsafe extern "C" fn point_x(L: *mut lua_State) -> c_int {
  let p = luaL_checkudata(L, 1, POINT_NATIVE) as *mut Point;
  lua_pushnumber(L, (*p).x);
  return 1;
}

unsafe extern "C" fn point_y(L: *mut lua_State) -> c_int {
  let p = luaL_checkudata(L, 1, POINT_NATIVE) as *mut Point;
  lua_pushnumber(L, (*p).y);
  return 1;
}

unsafe extern "C" fn point_setx(L: *mut lua_State) -> c_int {
  let p = luaL_checkudata(L, 1, POINT_NATIVE) as *mut Point;
  let new_x = luaL_checknumber(L, 2);
  (*p).x = new_x;
  return 0;
}

unsafe extern "C" fn point_sety(L: *mut lua_State) -> c_int {
  let p = luaL_checkudata(L, 1, POINT_NATIVE) as *mut Point;
  let new_y = luaL_checknumber(L, 2);
  (*p).y = new_y;
  return 0;
}

unsafe extern "C" fn point_add(L: *mut lua_State) -> c_int {
  let a = luaL_checkudata(L, 1, POINT_NATIVE) as *mut Point;
  let b = luaL_checkudata(L, 2, POINT_NATIVE) as *mut Point;

  lua_pushnumber(L, (*a).x + (*b).x);
  lua_pushnumber(L, (*a).y + (*b).y);
  return point_new(L);
}

unsafe extern "C" fn point_sub(L: *mut lua_State) -> c_int {
  let a = luaL_checkudata(L, 1, POINT_NATIVE) as *mut Point;
  let b = luaL_checkudata(L, 2, POINT_NATIVE) as *mut Point;

  lua_pushnumber(L, (*a).x - (*b).x);
  lua_pushnumber(L, (*a).y - (*b).y);
  return point_new(L);
}

unsafe extern "C" fn point_dist(L: *mut lua_State) -> c_int {
  let a = luaL_checkudata(L, 1, POINT_NATIVE) as *mut Point;
  let b = luaL_checkudata(L, 2, POINT_NATIVE) as *mut Point;
  let x_diff = (*a).x - (*b).x;
  let y_diff = (*a).y - (*b).y;
  let squared = f64::powf(x_diff, 2.0) + f64::powf(y_diff, 2.0);
  lua_pushnumber(L, f64::powf(squared, 0.5));
  return 1;
}

unsafe extern "C" fn point_tostring(L: *mut lua_State) -> c_int {
  let p = luaL_checkudata(L, 1, POINT_NATIVE) as *mut Point;
  let string = format!("{{{}, {}}}", (*p).x, (*p).y);
  lua_pushstring(L, cstr(string));
  return 1;
}

unsafe extern "C" fn point_eq(L: *mut lua_State) -> c_int {
  let a = luaL_checkudata(L, 1, POINT_NATIVE) as *mut Point;
  let b = luaL_checkudata(L, 2, POINT_NATIVE) as *mut Point;
  lua_pushboolean(L, if (*a).x == (*b).x && (*a).y == (*b).y { 1 } else { 0 });
  return 1;
}

#[no_mangle]
pub unsafe extern "C" fn luaopen_point_native_point(L: *mut lua_State) -> isize {
  POINT_NATIVE = CString::new("point_native").unwrap().into_raw();

  let pushfn = |s, f| {
    lua_pushstring(L, cstr(s));
    lua_pushcfunction(L, Some(f));
    lua_settable(L, -3);
  };

  luaL_newmetatable(L as *mut lauxlib::lua_State, POINT_NATIVE);
  pushfn("__add", point_add);
  pushfn("__sub", point_sub);
  pushfn("__eq", point_eq);
  pushfn("__tostring", point_tostring);

  lua_createtable(L, 1, 0);
  pushfn("new", point_new);
  pushfn("x", point_x);
  pushfn("y", point_y);
  pushfn("set_x", point_setx);
  pushfn("set_y", point_sety);
  pushfn("dist", point_dist);

  lua_pushstring(L, cstr("__index"));
  lua_pushvalue(L, -2);
  lua_settable(L, -4);

  return 1;
}
