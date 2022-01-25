#include "../../include/variant.h"
#include "cxx.h"
#include <array>
#include <cstddef>
#include <cstdint>
#include <memory>
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

namespace {
template <bool> struct deleter_if {
  template <typename T> void operator()(T *) {}
};

template <> struct deleter_if<true> {
  template <typename T> void operator()(T *ptr) { ptr->~T(); }
};
} // namespace
} // namespace cxxbridge1
} // namespace rust

using Variant = ::Variant;

static_assert(
    ::rust::IsRelocatable<::VariantType>::value,
    "type VariantType should be trivially move constructible and trivially destructible in C++ to be used as a return value of `get_type` in Rust");

extern "C" {
void cxxbridge1$Variant$get_type(const ::Variant &self, ::VariantType *return$) noexcept {
  ::VariantType (::Variant::*get_type$)() const = &::Variant::get_type;
  new (return$) ::VariantType((self.*get_type$)());
}

void cxxbridge1$yes_cxx_variant_can_be_a_unique_ptr_target(::Variant *variant) noexcept {
  void (*yes_cxx_variant_can_be_a_unique_ptr_target$)(::std::unique_ptr<::Variant>) = ::yes_cxx_variant_can_be_a_unique_ptr_target;
  yes_cxx_variant_can_be_a_unique_ptr_target$(::std::unique_ptr<::Variant>(variant));
}

::std::int64_t cxxbridge1$variant_as_i64(const ::Variant &variant) noexcept {
  ::std::int64_t (*variant_as_i64$)(const ::Variant &) = ::variant_as_i64;
  return variant_as_i64$(variant);
}

void cxxbridge1$variant_as_string(const ::Variant &variant, ::rust::String *return$) noexcept {
  ::rust::String (*variant_as_string$)(const ::Variant &) = ::variant_as_string;
  new (return$) ::rust::String(variant_as_string$(variant));
}

bool cxxbridge1$variant_as_bool(const ::Variant &variant) noexcept {
  bool (*variant_as_bool$)(const ::Variant &) = ::variant_as_bool;
  return variant_as_bool$(variant);
}

double cxxbridge1$variant_as_f64(const ::Variant &variant) noexcept {
  double (*variant_as_f64$)(const ::Variant &) = ::variant_as_f64;
  return variant_as_f64$(variant);
}

static_assert(::rust::detail::is_complete<::Variant>::value, "definition of Variant is required");
static_assert(sizeof(::std::unique_ptr<::Variant>) == sizeof(void *), "");
static_assert(alignof(::std::unique_ptr<::Variant>) == alignof(void *), "");
void cxxbridge1$unique_ptr$Variant$null(::std::unique_ptr<::Variant> *ptr) noexcept {
  ::new (ptr) ::std::unique_ptr<::Variant>();
}
void cxxbridge1$unique_ptr$Variant$raw(::std::unique_ptr<::Variant> *ptr, ::Variant *raw) noexcept {
  ::new (ptr) ::std::unique_ptr<::Variant>(raw);
}
const ::Variant *cxxbridge1$unique_ptr$Variant$get(const ::std::unique_ptr<::Variant>& ptr) noexcept {
  return ptr.get();
}
::Variant *cxxbridge1$unique_ptr$Variant$release(::std::unique_ptr<::Variant>& ptr) noexcept {
  return ptr.release();
}
void cxxbridge1$unique_ptr$Variant$drop(::std::unique_ptr<::Variant> *ptr) noexcept {
  ::rust::deleter_if<::rust::detail::is_complete<::Variant>::value>{}(ptr);
}
} // extern "C"
