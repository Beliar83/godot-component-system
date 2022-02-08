#include "variant.rs.h"
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

namespace gcs {
  namespace ffi {
    struct ComponentFieldDefinition;
  }
}

namespace gcs {
namespace ffi {
#ifndef CXXBRIDGE1_STRUCT_gcs$ffi$ComponentFieldDefinition
#define CXXBRIDGE1_STRUCT_gcs$ffi$ComponentFieldDefinition
struct ComponentFieldDefinition final {
  ::rust::String name;
  ::gcs::ffi::VariantType field_type;

  bool operator==(const ComponentFieldDefinition &) const noexcept;
  bool operator!=(const ComponentFieldDefinition &) const noexcept;
  using IsRelocatable = ::std::true_type;
};
#endif // CXXBRIDGE1_STRUCT_gcs$ffi$ComponentFieldDefinition
} // namespace ffi
} // namespace gcs

static_assert(
    ::rust::IsRelocatable<::gcs::ffi::VariantType>::value,
    "type gcs::ffi::VariantType should be trivially move constructible and trivially destructible in C++ to be used as a field of `ComponentFieldDefinition` or argument of `create_component_field_definition` in Rust");

namespace gcs {
namespace ffi {
extern "C" {
bool gcs$ffi$cxxbridge1$ComponentFieldDefinition$operator$eq(const ComponentFieldDefinition &, const ComponentFieldDefinition &) noexcept;
::std::size_t gcs$ffi$cxxbridge1$ComponentFieldDefinition$operator$hash(const ComponentFieldDefinition &) noexcept;

void gcs$ffi$cxxbridge1$create_component_field_definition(::rust::String *name, ::gcs::ffi::VariantType *field_type, ::gcs::ffi::ComponentFieldDefinition *return$) noexcept;
} // extern "C"
} // namespace ffi
} // namespace gcs

namespace std {
template <> struct hash<::gcs::ffi::ComponentFieldDefinition> {
  ::std::size_t operator()(const ::gcs::ffi::ComponentFieldDefinition &self) const noexcept {
    return ::gcs::ffi::gcs$ffi$cxxbridge1$ComponentFieldDefinition$operator$hash(self);
  }
};
} // namespace std

namespace gcs {
namespace ffi {
bool ComponentFieldDefinition::operator==(const ComponentFieldDefinition &rhs) const noexcept {
  return gcs$ffi$cxxbridge1$ComponentFieldDefinition$operator$eq(*this, rhs);
}

bool ComponentFieldDefinition::operator!=(const ComponentFieldDefinition &rhs) const noexcept {
  return !(*this == rhs);
}

::gcs::ffi::ComponentFieldDefinition create_component_field_definition(::rust::String name, ::gcs::ffi::VariantType field_type) noexcept {
  ::rust::ManuallyDrop<::gcs::ffi::VariantType> field_type$(::std::move(field_type));
  ::rust::MaybeUninit<::gcs::ffi::ComponentFieldDefinition> return$;
  gcs$ffi$cxxbridge1$create_component_field_definition(&name, &field_type$.value, &return$.value);
  return ::std::move(return$.value);
}
} // namespace ffi
} // namespace gcs
