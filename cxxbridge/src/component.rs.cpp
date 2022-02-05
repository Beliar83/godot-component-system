#include "component.h"
#include "cxx.h"
#include <array>
#include <cstddef>
#include <cstdint>
#include <functional>
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

#ifndef CXXBRIDGE1_RELOCATABLE
#define CXXBRIDGE1_RELOCATABLE
namespace detail {
template <typename... Ts>
struct make_void {
  using type = void;
};

template <typename... Ts>
using void_t = typename make_void<Ts...>::type;

template <typename Void, template <typename...> class, typename...>
struct detect : std::false_type {};
template <template <typename...> class T, typename... A>
struct detect<void_t<T<A...>>, T, A...> : std::true_type {};

template <template <typename...> class T, typename... A>
using is_detected = detect<void, T, A...>;

template <typename T>
using detect_IsRelocatable = typename T::IsRelocatable;

template <typename T>
struct get_IsRelocatable
    : std::is_same<typename T::IsRelocatable, std::true_type> {};
} // namespace detail

template <typename T>
struct IsRelocatable
    : std::conditional<
          detail::is_detected<detail::detect_IsRelocatable, T>::value,
          detail::get_IsRelocatable<T>,
          std::integral_constant<
              bool, std::is_trivially_move_constructible<T>::value &&
                        std::is_trivially_destructible<T>::value>>::type {};
#endif // CXXBRIDGE1_RELOCATABLE

namespace detail {
template <typename T, typename = void *>
struct operator_new {
  void *operator()(::std::size_t sz) { return ::operator new(sz); }
};

template <typename T>
struct operator_new<T, decltype(T::operator new(sizeof(T)))> {
  void *operator()(::std::size_t sz) { return T::operator new(sz); }
};
} // namespace detail

template <typename T>
union ManuallyDrop {
  T value;
  ManuallyDrop(T &&value) : value(::std::move(value)) {}
  ~ManuallyDrop() {}
};

template <typename T>
union MaybeUninit {
  T value;
  void *operator new(::std::size_t sz) { return detail::operator_new<T>{}(sz); }
  MaybeUninit() {}
  ~MaybeUninit() {}
};
} // namespace cxxbridge1
} // namespace rust

struct ComponentFieldDefinition;
struct ComponentData;
struct ComponentValue;

#ifndef CXXBRIDGE1_STRUCT_ComponentFieldDefinition
#define CXXBRIDGE1_STRUCT_ComponentFieldDefinition
struct ComponentFieldDefinition final {
  ::rust::String name;
  ::VariantType field_type;

  bool operator==(const ComponentFieldDefinition &) const noexcept;
  bool operator!=(const ComponentFieldDefinition &) const noexcept;
  using IsRelocatable = ::std::true_type;
};
#endif // CXXBRIDGE1_STRUCT_ComponentFieldDefinition

#ifndef CXXBRIDGE1_STRUCT_ComponentData
#define CXXBRIDGE1_STRUCT_ComponentData
struct ComponentData final : public ::rust::Opaque {
  const ::ComponentValue &get_field(::rust::String field) const noexcept;
  void set_field(::rust::String field, const ::ComponentValue &value) noexcept;
  ~ComponentData() = delete;

private:
  friend ::rust::layout;
  struct layout {
    static ::std::size_t size() noexcept;
    static ::std::size_t align() noexcept;
  };
};
#endif // CXXBRIDGE1_STRUCT_ComponentData

#ifndef CXXBRIDGE1_STRUCT_ComponentValue
#define CXXBRIDGE1_STRUCT_ComponentValue
struct ComponentValue final : public ::rust::Opaque {
  ~ComponentValue() = delete;

private:
  friend ::rust::layout;
  struct layout {
    static ::std::size_t size() noexcept;
    static ::std::size_t align() noexcept;
  };
};
#endif // CXXBRIDGE1_STRUCT_ComponentValue

static_assert(
    ::rust::IsRelocatable<::VariantType>::value,
    "type VariantType should be trivially move constructible and trivially destructible in C++ to be used as a field of `ComponentFieldDefinition` or argument of `create_component_field_definition` in Rust");

extern "C" {
bool cxxbridge1$ComponentFieldDefinition$operator$eq(const ComponentFieldDefinition &, const ComponentFieldDefinition &) noexcept;
::std::size_t cxxbridge1$ComponentFieldDefinition$operator$hash(const ComponentFieldDefinition &) noexcept;

void cxxbridge1$create_component_field_definition(::rust::String *name, ::VariantType *field_type, ::ComponentFieldDefinition *return$) noexcept;
::std::size_t cxxbridge1$ComponentData$operator$sizeof() noexcept;
::std::size_t cxxbridge1$ComponentData$operator$alignof() noexcept;
::std::size_t cxxbridge1$ComponentValue$operator$sizeof() noexcept;
::std::size_t cxxbridge1$ComponentValue$operator$alignof() noexcept;

const ::ComponentValue *cxxbridge1$ComponentData$get_field(const ::ComponentData &self, ::rust::String *field) noexcept;

void cxxbridge1$ComponentData$set_field(::ComponentData &self, ::rust::String *field, const ::ComponentValue &value) noexcept;

const ::Variant *cxxbridge1$variant_from_component_value(const ::ComponentValue &value) noexcept;

const ::Variant *cxxbridge1$empty_variant() noexcept {
  const ::Variant &(*empty_variant$)() = ::empty_variant;
  return &empty_variant$();
}

const ::Variant *cxxbridge1$variant_from_i64(::std::int64_t value) noexcept {
  const ::Variant &(*variant_from_i64$)(::std::int64_t) = ::variant_from_i64;
  return &variant_from_i64$(value);
}

const ::Variant *cxxbridge1$variant_from_string(const ::rust::String *value) noexcept {
  const ::Variant &(*variant_from_string$)(::rust::String) = ::variant_from_string;
  return &variant_from_string$(::rust::String(::rust::unsafe_bitcopy, *value));
}

const ::Variant *cxxbridge1$variant_from_bool(bool value) noexcept {
  const ::Variant &(*variant_from_bool$)(bool) = ::variant_from_bool;
  return &variant_from_bool$(value);
}

const ::Variant *cxxbridge1$variant_from_f64(double value) noexcept {
  const ::Variant &(*variant_from_f64$)(double) = ::variant_from_f64;
  return &variant_from_f64$(value);
}
} // extern "C"

namespace std {
template <> struct hash<::ComponentFieldDefinition> {
  ::std::size_t operator()(const ::ComponentFieldDefinition &self) const noexcept {
    return ::cxxbridge1$ComponentFieldDefinition$operator$hash(self);
  }
};
} // namespace std

bool ComponentFieldDefinition::operator==(const ComponentFieldDefinition &rhs) const noexcept {
  return cxxbridge1$ComponentFieldDefinition$operator$eq(*this, rhs);
}

bool ComponentFieldDefinition::operator!=(const ComponentFieldDefinition &rhs) const noexcept {
  return !(*this == rhs);
}

::ComponentFieldDefinition create_component_field_definition(::rust::String name, ::VariantType field_type) noexcept {
  ::rust::ManuallyDrop<::VariantType> field_type$(::std::move(field_type));
  ::rust::MaybeUninit<::ComponentFieldDefinition> return$;
  cxxbridge1$create_component_field_definition(&name, &field_type$.value, &return$.value);
  return ::std::move(return$.value);
}

::std::size_t ComponentData::layout::size() noexcept {
  return cxxbridge1$ComponentData$operator$sizeof();
}

::std::size_t ComponentData::layout::align() noexcept {
  return cxxbridge1$ComponentData$operator$alignof();
}

::std::size_t ComponentValue::layout::size() noexcept {
  return cxxbridge1$ComponentValue$operator$sizeof();
}

::std::size_t ComponentValue::layout::align() noexcept {
  return cxxbridge1$ComponentValue$operator$alignof();
}

const ::ComponentValue &ComponentData::get_field(::rust::String field) const noexcept {
  return *cxxbridge1$ComponentData$get_field(*this, &field);
}

void ComponentData::set_field(::rust::String field, const ::ComponentValue &value) noexcept {
  cxxbridge1$ComponentData$set_field(*this, &field, value);
}

const ::Variant &variant_from_component_value(const ::ComponentValue &value) noexcept {
  return *cxxbridge1$variant_from_component_value(value);
}
