#include "component.h"
#include "cxx.h"
#include <array>
#include <cstddef>
#include <cstdint>
#include <new>
#include <string>
#include <type_traits>
#include <utility>

namespace rust {
inline namespace cxxbridge1 {
// #include "rust/cxx.h"

struct unsafe_bitcopy_t;

#ifndef CXXBRIDGE1_RUST_STRING
#define CXXBRIDGE1_RUST_STRING
class String final {
public:
  String() noexcept;
  String(const String &) noexcept;
  String(String &&) noexcept;
  ~String() noexcept;

  String(const std::string &);
  String(const char *);
  String(const char *, std::size_t);
  String(const char16_t *);
  String(const char16_t *, std::size_t);

  static String lossy(const std::string &) noexcept;
  static String lossy(const char *) noexcept;
  static String lossy(const char *, std::size_t) noexcept;
  static String lossy(const char16_t *) noexcept;
  static String lossy(const char16_t *, std::size_t) noexcept;

  String &operator=(const String &) &noexcept;
  String &operator=(String &&) &noexcept;

  explicit operator std::string() const;

  const char *data() const noexcept;
  std::size_t size() const noexcept;
  std::size_t length() const noexcept;
  bool empty() const noexcept;

  const char *c_str() noexcept;

  std::size_t capacity() const noexcept;
  void reserve(size_t new_cap) noexcept;

  using iterator = char *;
  iterator begin() noexcept;
  iterator end() noexcept;

  using const_iterator = const char *;
  const_iterator begin() const noexcept;
  const_iterator end() const noexcept;
  const_iterator cbegin() const noexcept;
  const_iterator cend() const noexcept;

  bool operator==(const String &) const noexcept;
  bool operator!=(const String &) const noexcept;
  bool operator<(const String &) const noexcept;
  bool operator<=(const String &) const noexcept;
  bool operator>(const String &) const noexcept;
  bool operator>=(const String &) const noexcept;

  void swap(String &) noexcept;

  String(unsafe_bitcopy_t, const String &) noexcept;

private:
  struct lossy_t;
  String(lossy_t, const char *, std::size_t) noexcept;
  String(lossy_t, const char16_t *, std::size_t) noexcept;
  friend void swap(String &lhs, String &rhs) noexcept { lhs.swap(rhs); }

  std::array<std::uintptr_t, 3> repr;
};
#endif // CXXBRIDGE1_RUST_STRING

#ifndef CXXBRIDGE1_RUST_BOX
#define CXXBRIDGE1_RUST_BOX
template <typename T>
class Box final {
public:
  using element_type = T;
  using const_pointer =
      typename std::add_pointer<typename std::add_const<T>::type>::type;
  using pointer = typename std::add_pointer<T>::type;

  Box() = delete;
  Box(Box &&) noexcept;
  ~Box() noexcept;

  explicit Box(const T &);
  explicit Box(T &&);

  Box &operator=(Box &&) &noexcept;

  const T *operator->() const noexcept;
  const T &operator*() const noexcept;
  T *operator->() noexcept;
  T &operator*() noexcept;

  template <typename... Fields>
  static Box in_place(Fields &&...);

  void swap(Box &) noexcept;

  static Box from_raw(T *) noexcept;

  T *into_raw() noexcept;

  /* Deprecated */ using value_type = element_type;

private:
  class uninit;
  class allocation;
  Box(uninit) noexcept;
  void drop() noexcept;

  friend void swap(Box &lhs, Box &rhs) noexcept { lhs.swap(rhs); }

  T *ptr;
};

template <typename T>
class Box<T>::uninit {};

template <typename T>
class Box<T>::allocation {
  static T *alloc() noexcept;
  static void dealloc(T *) noexcept;

public:
  allocation() noexcept : ptr(alloc()) {}
  ~allocation() noexcept {
    if (this->ptr) {
      dealloc(this->ptr);
    }
  }
  T *ptr;
};

template <typename T>
Box<T>::Box(Box &&other) noexcept : ptr(other.ptr) {
  other.ptr = nullptr;
}

template <typename T>
Box<T>::Box(const T &val) {
  allocation alloc;
  ::new (alloc.ptr) T(val);
  this->ptr = alloc.ptr;
  alloc.ptr = nullptr;
}

template <typename T>
Box<T>::Box(T &&val) {
  allocation alloc;
  ::new (alloc.ptr) T(std::move(val));
  this->ptr = alloc.ptr;
  alloc.ptr = nullptr;
}

template <typename T>
Box<T>::~Box() noexcept {
  if (this->ptr) {
    this->drop();
  }
}

template <typename T>
Box<T> &Box<T>::operator=(Box &&other) &noexcept {
  if (this->ptr) {
    this->drop();
  }
  this->ptr = other.ptr;
  other.ptr = nullptr;
  return *this;
}

template <typename T>
const T *Box<T>::operator->() const noexcept {
  return this->ptr;
}

template <typename T>
const T &Box<T>::operator*() const noexcept {
  return *this->ptr;
}

template <typename T>
T *Box<T>::operator->() noexcept {
  return this->ptr;
}

template <typename T>
T &Box<T>::operator*() noexcept {
  return *this->ptr;
}

template <typename T>
template <typename... Fields>
Box<T> Box<T>::in_place(Fields &&...fields) {
  allocation alloc;
  auto ptr = alloc.ptr;
  ::new (ptr) T{std::forward<Fields>(fields)...};
  alloc.ptr = nullptr;
  return from_raw(ptr);
}

template <typename T>
void Box<T>::swap(Box &rhs) noexcept {
  using std::swap;
  swap(this->ptr, rhs.ptr);
}

template <typename T>
Box<T> Box<T>::from_raw(T *raw) noexcept {
  Box box = uninit{};
  box.ptr = raw;
  return box;
}

template <typename T>
T *Box<T>::into_raw() noexcept {
  T *raw = this->ptr;
  this->ptr = nullptr;
  return raw;
}

template <typename T>
Box<T>::Box(uninit) noexcept {}
#endif // CXXBRIDGE1_RUST_BOX

#ifndef CXXBRIDGE1_RUST_BITCOPY_T
#define CXXBRIDGE1_RUST_BITCOPY_T
struct unsafe_bitcopy_t final {
  explicit unsafe_bitcopy_t() = default;
};
#endif // CXXBRIDGE1_RUST_BITCOPY_T

#ifndef CXXBRIDGE1_RUST_BITCOPY
#define CXXBRIDGE1_RUST_BITCOPY
constexpr unsafe_bitcopy_t unsafe_bitcopy{};
#endif // CXXBRIDGE1_RUST_BITCOPY

#ifndef CXXBRIDGE1_RUST_OPAQUE
#define CXXBRIDGE1_RUST_OPAQUE
class Opaque {
public:
  Opaque() = delete;
  Opaque(const Opaque &) = delete;
  ~Opaque() = delete;
};
#endif // CXXBRIDGE1_RUST_OPAQUE

#ifndef CXXBRIDGE1_IS_COMPLETE
#define CXXBRIDGE1_IS_COMPLETE
namespace detail {
namespace {
template <typename T, typename = std::size_t>
struct is_complete : std::false_type {};
template <typename T>
struct is_complete<T, decltype(sizeof(T))> : std::true_type {};
} // namespace
} // namespace detail
#endif // CXXBRIDGE1_IS_COMPLETE

#ifndef CXXBRIDGE1_LAYOUT
#define CXXBRIDGE1_LAYOUT
class layout {
  template <typename T>
  friend std::size_t size_of();
  template <typename T>
  friend std::size_t align_of();
  template <typename T>
  static typename std::enable_if<std::is_base_of<Opaque, T>::value,
                                 std::size_t>::type
  do_size_of() {
    return T::layout::size();
  }
  template <typename T>
  static typename std::enable_if<!std::is_base_of<Opaque, T>::value,
                                 std::size_t>::type
  do_size_of() {
    return sizeof(T);
  }
  template <typename T>
  static
      typename std::enable_if<detail::is_complete<T>::value, std::size_t>::type
      size_of() {
    return do_size_of<T>();
  }
  template <typename T>
  static typename std::enable_if<std::is_base_of<Opaque, T>::value,
                                 std::size_t>::type
  do_align_of() {
    return T::layout::align();
  }
  template <typename T>
  static typename std::enable_if<!std::is_base_of<Opaque, T>::value,
                                 std::size_t>::type
  do_align_of() {
    return alignof(T);
  }
  template <typename T>
  static
      typename std::enable_if<detail::is_complete<T>::value, std::size_t>::type
      align_of() {
    return do_align_of<T>();
  }
};

template <typename T>
std::size_t size_of() {
  return layout::size_of<T>();
}

template <typename T>
std::size_t align_of() {
  return layout::align_of<T>();
}
#endif // CXXBRIDGE1_LAYOUT
} // namespace cxxbridge1
} // namespace rust

namespace gcs {
  namespace ffi {
    struct ComponentValue;
  }
}

namespace gcs {
namespace ffi {
#ifndef CXXBRIDGE1_STRUCT_gcs$ffi$ComponentValue
#define CXXBRIDGE1_STRUCT_gcs$ffi$ComponentValue
struct ComponentValue final : public ::rust::Opaque {
  ~ComponentValue() = delete;

private:
  friend ::rust::layout;
  struct layout {
    static ::std::size_t size() noexcept;
    static ::std::size_t align() noexcept;
  };
};
#endif // CXXBRIDGE1_STRUCT_gcs$ffi$ComponentValue

extern "C" {
::std::size_t gcs$ffi$cxxbridge1$ComponentValue$operator$sizeof() noexcept;
::std::size_t gcs$ffi$cxxbridge1$ComponentValue$operator$alignof() noexcept;

const ::gcs::ffi::Variant *gcs$ffi$cxxbridge1$variant_from_component_value(const ::gcs::ffi::ComponentValue &value) noexcept;

::gcs::ffi::ComponentValue *gcs$ffi$cxxbridge1$component_value_from_variant(const ::gcs::ffi::Variant &value) noexcept;

const ::gcs::ffi::Variant *gcs$ffi$cxxbridge1$empty_variant() noexcept {
  const ::gcs::ffi::Variant &(*empty_variant$)() = ::gcs::ffi::empty_variant;
  return &empty_variant$();
}

const ::gcs::ffi::Variant *gcs$ffi$cxxbridge1$variant_from_i64(::std::int64_t value) noexcept {
  const ::gcs::ffi::Variant &(*variant_from_i64$)(::std::int64_t) = ::gcs::ffi::variant_from_i64;
  return &variant_from_i64$(value);
}

const ::gcs::ffi::Variant *gcs$ffi$cxxbridge1$variant_from_string(const ::rust::String *value) noexcept {
  const ::gcs::ffi::Variant &(*variant_from_string$)(::rust::String) = ::gcs::ffi::variant_from_string;
  return &variant_from_string$(::rust::String(::rust::unsafe_bitcopy, *value));
}

const ::gcs::ffi::Variant *gcs$ffi$cxxbridge1$variant_from_bool(bool value) noexcept {
  const ::gcs::ffi::Variant &(*variant_from_bool$)(bool) = ::gcs::ffi::variant_from_bool;
  return &variant_from_bool$(value);
}

const ::gcs::ffi::Variant *gcs$ffi$cxxbridge1$variant_from_f64(double value) noexcept {
  const ::gcs::ffi::Variant &(*variant_from_f64$)(double) = ::gcs::ffi::variant_from_f64;
  return &variant_from_f64$(value);
}
} // extern "C"

::std::size_t ComponentValue::layout::size() noexcept {
  return gcs$ffi$cxxbridge1$ComponentValue$operator$sizeof();
}

::std::size_t ComponentValue::layout::align() noexcept {
  return gcs$ffi$cxxbridge1$ComponentValue$operator$alignof();
}

const ::gcs::ffi::Variant &variant_from_component_value(const ::gcs::ffi::ComponentValue &value) noexcept {
  return *gcs$ffi$cxxbridge1$variant_from_component_value(value);
}

::rust::Box<::gcs::ffi::ComponentValue> component_value_from_variant(const ::gcs::ffi::Variant &value) noexcept {
  return ::rust::Box<::gcs::ffi::ComponentValue>::from_raw(gcs$ffi$cxxbridge1$component_value_from_variant(value));
}
} // namespace ffi
} // namespace gcs

extern "C" {
::gcs::ffi::ComponentValue *cxxbridge1$box$gcs$ffi$ComponentValue$alloc() noexcept;
void cxxbridge1$box$gcs$ffi$ComponentValue$dealloc(::gcs::ffi::ComponentValue *) noexcept;
void cxxbridge1$box$gcs$ffi$ComponentValue$drop(::rust::Box<::gcs::ffi::ComponentValue> *ptr) noexcept;
} // extern "C"

namespace rust {
inline namespace cxxbridge1 {
template <>
::gcs::ffi::ComponentValue *Box<::gcs::ffi::ComponentValue>::allocation::alloc() noexcept {
  return cxxbridge1$box$gcs$ffi$ComponentValue$alloc();
}
template <>
void Box<::gcs::ffi::ComponentValue>::allocation::dealloc(::gcs::ffi::ComponentValue *ptr) noexcept {
  cxxbridge1$box$gcs$ffi$ComponentValue$dealloc(ptr);
}
template <>
void Box<::gcs::ffi::ComponentValue>::drop() noexcept {
  cxxbridge1$box$gcs$ffi$ComponentValue$drop(this);
}
} // namespace cxxbridge1
} // namespace rust
