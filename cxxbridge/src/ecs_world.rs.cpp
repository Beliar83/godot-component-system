#include "component_value.rs.h"
#include "cxx.h"
#include "cxx.h"
#include "godot/variant.h"
#include "component_definition.rs.h"
#include "component_value.rs.h"
#include <array>
#include <cstddef>
#include <cstdint>
#include <exception>
#include <new>
#include <string>
#include <type_traits>
#include <utility>

namespace rust {
inline namespace cxxbridge1 {
// #include "rust/cxx.h"

struct unsafe_bitcopy_t;

namespace {
template <typename T>
class impl;
} // namespace

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

#ifndef CXXBRIDGE1_RUST_ERROR
#define CXXBRIDGE1_RUST_ERROR
class Error final : public std::exception {
public:
  Error(const Error &);
  Error(Error &&) noexcept;
  ~Error() noexcept override;

  Error &operator=(const Error &) &;
  Error &operator=(Error &&) &noexcept;

  const char *what() const noexcept override;

private:
  Error() noexcept = default;
  friend impl<Error>;
  const char *msg;
  std::size_t len;
};
#endif // CXXBRIDGE1_RUST_ERROR

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
union MaybeUninit {
  T value;
  void *operator new(::std::size_t sz) { return detail::operator_new<T>{}(sz); }
  MaybeUninit() {}
  ~MaybeUninit() {}
};

namespace {
namespace repr {
struct PtrLen final {
  void *ptr;
  ::std::size_t len;
};
} // namespace repr

template <>
class impl<Error> final {
public:
  static Error error(repr::PtrLen repr) noexcept {
    Error error;
    error.msg = static_cast<const char *>(repr.ptr);
    error.len = repr.len;
    return error;
  }
};
} // namespace
} // namespace cxxbridge1
} // namespace rust

namespace gcs {
  namespace ffi {
    struct ComponentInfo;
    struct ComponentData;
    struct EntityId;
    struct ECSWorld;
  }
}

namespace gcs {
namespace ffi {
#ifndef CXXBRIDGE1_STRUCT_gcs$ffi$ComponentInfo
#define CXXBRIDGE1_STRUCT_gcs$ffi$ComponentInfo
struct ComponentInfo final : public ::rust::Opaque {
  ~ComponentInfo() = delete;

private:
  friend ::rust::layout;
  struct layout {
    static ::std::size_t size() noexcept;
    static ::std::size_t align() noexcept;
  };
};
#endif // CXXBRIDGE1_STRUCT_gcs$ffi$ComponentInfo

#ifndef CXXBRIDGE1_STRUCT_gcs$ffi$ComponentData
#define CXXBRIDGE1_STRUCT_gcs$ffi$ComponentData
struct ComponentData final : public ::rust::Opaque {
  const ::gcs::ffi::ComponentValue &get_field(::rust::String field) const noexcept;
  void set_field(::rust::String field, const ::gcs::ffi::ComponentValue &value) noexcept;
  ~ComponentData() = delete;

private:
  friend ::rust::layout;
  struct layout {
    static ::std::size_t size() noexcept;
    static ::std::size_t align() noexcept;
  };
};
#endif // CXXBRIDGE1_STRUCT_gcs$ffi$ComponentData

#ifndef CXXBRIDGE1_STRUCT_gcs$ffi$EntityId
#define CXXBRIDGE1_STRUCT_gcs$ffi$EntityId
struct EntityId final : public ::rust::Opaque {
  ::rust::String as_string() const noexcept;
  ~EntityId() = delete;

private:
  friend ::rust::layout;
  struct layout {
    static ::std::size_t size() noexcept;
    static ::std::size_t align() noexcept;
  };
};
#endif // CXXBRIDGE1_STRUCT_gcs$ffi$EntityId

#ifndef CXXBRIDGE1_STRUCT_gcs$ffi$ECSWorld
#define CXXBRIDGE1_STRUCT_gcs$ffi$ECSWorld
struct ECSWorld final : public ::rust::Opaque {
  ::rust::Box<::gcs::ffi::ComponentInfo> register_component(::rust::String name, const ::gcs::ffi::ComponentDefinition &component_definition);
  void register_entity(const ::gcs::ffi::EntityId &id);
  void set_component_data(const ::gcs::ffi::EntityId &entity_id, ::rust::String component, const ::gcs::ffi::ComponentData &data);
  bool is_component_added_to_entity(const ::gcs::ffi::EntityId &entity_id, ::rust::String component) const noexcept;
  ::rust::Box<::gcs::ffi::EntityId> create_entity() noexcept;
  ~ECSWorld() = delete;

private:
  friend ::rust::layout;
  struct layout {
    static ::std::size_t size() noexcept;
    static ::std::size_t align() noexcept;
  };
};
#endif // CXXBRIDGE1_STRUCT_gcs$ffi$ECSWorld

extern "C" {
::std::size_t gcs$ffi$cxxbridge1$ComponentInfo$operator$sizeof() noexcept;
::std::size_t gcs$ffi$cxxbridge1$ComponentInfo$operator$alignof() noexcept;
::std::size_t gcs$ffi$cxxbridge1$ComponentData$operator$sizeof() noexcept;
::std::size_t gcs$ffi$cxxbridge1$ComponentData$operator$alignof() noexcept;

const ::gcs::ffi::ComponentValue *gcs$ffi$cxxbridge1$ComponentData$get_field(const ::gcs::ffi::ComponentData &self, ::rust::String *field) noexcept;

void gcs$ffi$cxxbridge1$ComponentData$set_field(::gcs::ffi::ComponentData &self, ::rust::String *field, const ::gcs::ffi::ComponentValue &value) noexcept;

::gcs::ffi::ComponentData *gcs$ffi$cxxbridge1$create_component_data(const ::gcs::ffi::EntityId &entity) noexcept;
::std::size_t gcs$ffi$cxxbridge1$EntityId$operator$sizeof() noexcept;
::std::size_t gcs$ffi$cxxbridge1$EntityId$operator$alignof() noexcept;

::gcs::ffi::EntityId *gcs$ffi$cxxbridge1$create_entity() noexcept;

void gcs$ffi$cxxbridge1$EntityId$as_string(const ::gcs::ffi::EntityId &self, ::rust::String *return$) noexcept;

::rust::repr::PtrLen gcs$ffi$cxxbridge1$entity_id_from_string(::rust::String *id, ::rust::Box<::gcs::ffi::EntityId> *return$) noexcept;
::std::size_t gcs$ffi$cxxbridge1$ECSWorld$operator$sizeof() noexcept;
::std::size_t gcs$ffi$cxxbridge1$ECSWorld$operator$alignof() noexcept;

::gcs::ffi::ComponentInfo *gcs$ffi$cxxbridge1$create_component_info(::std::uint64_t hash) noexcept;

::rust::repr::PtrLen gcs$ffi$cxxbridge1$ECSWorld$register_component(::gcs::ffi::ECSWorld &self, ::rust::String *name, const ::gcs::ffi::ComponentDefinition &component_definition, ::rust::Box<::gcs::ffi::ComponentInfo> *return$) noexcept;

::rust::repr::PtrLen gcs$ffi$cxxbridge1$ECSWorld$register_entity(::gcs::ffi::ECSWorld &self, const ::gcs::ffi::EntityId &id) noexcept;

::rust::repr::PtrLen gcs$ffi$cxxbridge1$ECSWorld$set_component_data(::gcs::ffi::ECSWorld &self, const ::gcs::ffi::EntityId &entity_id, ::rust::String *component, const ::gcs::ffi::ComponentData &data) noexcept;

bool gcs$ffi$cxxbridge1$ECSWorld$is_component_added_to_entity(const ::gcs::ffi::ECSWorld &self, const ::gcs::ffi::EntityId &entity_id, ::rust::String *component) noexcept;

::gcs::ffi::EntityId *gcs$ffi$cxxbridge1$ECSWorld$create_entity(::gcs::ffi::ECSWorld &self) noexcept;

::gcs::ffi::ECSWorld *gcs$ffi$cxxbridge1$create_ecs_world() noexcept;
} // extern "C"

::std::size_t ComponentInfo::layout::size() noexcept {
  return gcs$ffi$cxxbridge1$ComponentInfo$operator$sizeof();
}

::std::size_t ComponentInfo::layout::align() noexcept {
  return gcs$ffi$cxxbridge1$ComponentInfo$operator$alignof();
}

::std::size_t ComponentData::layout::size() noexcept {
  return gcs$ffi$cxxbridge1$ComponentData$operator$sizeof();
}

::std::size_t ComponentData::layout::align() noexcept {
  return gcs$ffi$cxxbridge1$ComponentData$operator$alignof();
}

const ::gcs::ffi::ComponentValue &ComponentData::get_field(::rust::String field) const noexcept {
  return *gcs$ffi$cxxbridge1$ComponentData$get_field(*this, &field);
}

void ComponentData::set_field(::rust::String field, const ::gcs::ffi::ComponentValue &value) noexcept {
  gcs$ffi$cxxbridge1$ComponentData$set_field(*this, &field, value);
}

::rust::Box<::gcs::ffi::ComponentData> create_component_data(const ::gcs::ffi::EntityId &entity) noexcept {
  return ::rust::Box<::gcs::ffi::ComponentData>::from_raw(gcs$ffi$cxxbridge1$create_component_data(entity));
}

::std::size_t EntityId::layout::size() noexcept {
  return gcs$ffi$cxxbridge1$EntityId$operator$sizeof();
}

::std::size_t EntityId::layout::align() noexcept {
  return gcs$ffi$cxxbridge1$EntityId$operator$alignof();
}

::rust::Box<::gcs::ffi::EntityId> create_entity() noexcept {
  return ::rust::Box<::gcs::ffi::EntityId>::from_raw(gcs$ffi$cxxbridge1$create_entity());
}

::rust::String EntityId::as_string() const noexcept {
  ::rust::MaybeUninit<::rust::String> return$;
  gcs$ffi$cxxbridge1$EntityId$as_string(*this, &return$.value);
  return ::std::move(return$.value);
}

::rust::Box<::gcs::ffi::EntityId> entity_id_from_string(::rust::String id) {
  ::rust::MaybeUninit<::rust::Box<::gcs::ffi::EntityId>> return$;
  ::rust::repr::PtrLen error$ = gcs$ffi$cxxbridge1$entity_id_from_string(&id, &return$.value);
  if (error$.ptr) {
    throw ::rust::impl<::rust::Error>::error(error$);
  }
  return ::std::move(return$.value);
}

::std::size_t ECSWorld::layout::size() noexcept {
  return gcs$ffi$cxxbridge1$ECSWorld$operator$sizeof();
}

::std::size_t ECSWorld::layout::align() noexcept {
  return gcs$ffi$cxxbridge1$ECSWorld$operator$alignof();
}

::rust::Box<::gcs::ffi::ComponentInfo> create_component_info(::std::uint64_t hash) noexcept {
  return ::rust::Box<::gcs::ffi::ComponentInfo>::from_raw(gcs$ffi$cxxbridge1$create_component_info(hash));
}

::rust::Box<::gcs::ffi::ComponentInfo> ECSWorld::register_component(::rust::String name, const ::gcs::ffi::ComponentDefinition &component_definition) {
  ::rust::MaybeUninit<::rust::Box<::gcs::ffi::ComponentInfo>> return$;
  ::rust::repr::PtrLen error$ = gcs$ffi$cxxbridge1$ECSWorld$register_component(*this, &name, component_definition, &return$.value);
  if (error$.ptr) {
    throw ::rust::impl<::rust::Error>::error(error$);
  }
  return ::std::move(return$.value);
}

void ECSWorld::register_entity(const ::gcs::ffi::EntityId &id) {
  ::rust::repr::PtrLen error$ = gcs$ffi$cxxbridge1$ECSWorld$register_entity(*this, id);
  if (error$.ptr) {
    throw ::rust::impl<::rust::Error>::error(error$);
  }
}

void ECSWorld::set_component_data(const ::gcs::ffi::EntityId &entity_id, ::rust::String component, const ::gcs::ffi::ComponentData &data) {
  ::rust::repr::PtrLen error$ = gcs$ffi$cxxbridge1$ECSWorld$set_component_data(*this, entity_id, &component, data);
  if (error$.ptr) {
    throw ::rust::impl<::rust::Error>::error(error$);
  }
}

bool ECSWorld::is_component_added_to_entity(const ::gcs::ffi::EntityId &entity_id, ::rust::String component) const noexcept {
  return gcs$ffi$cxxbridge1$ECSWorld$is_component_added_to_entity(*this, entity_id, &component);
}

::rust::Box<::gcs::ffi::EntityId> ECSWorld::create_entity() noexcept {
  return ::rust::Box<::gcs::ffi::EntityId>::from_raw(gcs$ffi$cxxbridge1$ECSWorld$create_entity(*this));
}

::rust::Box<::gcs::ffi::ECSWorld> create_ecs_world() noexcept {
  return ::rust::Box<::gcs::ffi::ECSWorld>::from_raw(gcs$ffi$cxxbridge1$create_ecs_world());
}
} // namespace ffi
} // namespace gcs

extern "C" {
::gcs::ffi::ComponentData *cxxbridge1$box$gcs$ffi$ComponentData$alloc() noexcept;
void cxxbridge1$box$gcs$ffi$ComponentData$dealloc(::gcs::ffi::ComponentData *) noexcept;
void cxxbridge1$box$gcs$ffi$ComponentData$drop(::rust::Box<::gcs::ffi::ComponentData> *ptr) noexcept;

::gcs::ffi::EntityId *cxxbridge1$box$gcs$ffi$EntityId$alloc() noexcept;
void cxxbridge1$box$gcs$ffi$EntityId$dealloc(::gcs::ffi::EntityId *) noexcept;
void cxxbridge1$box$gcs$ffi$EntityId$drop(::rust::Box<::gcs::ffi::EntityId> *ptr) noexcept;

::gcs::ffi::ComponentInfo *cxxbridge1$box$gcs$ffi$ComponentInfo$alloc() noexcept;
void cxxbridge1$box$gcs$ffi$ComponentInfo$dealloc(::gcs::ffi::ComponentInfo *) noexcept;
void cxxbridge1$box$gcs$ffi$ComponentInfo$drop(::rust::Box<::gcs::ffi::ComponentInfo> *ptr) noexcept;

::gcs::ffi::ECSWorld *cxxbridge1$box$gcs$ffi$ECSWorld$alloc() noexcept;
void cxxbridge1$box$gcs$ffi$ECSWorld$dealloc(::gcs::ffi::ECSWorld *) noexcept;
void cxxbridge1$box$gcs$ffi$ECSWorld$drop(::rust::Box<::gcs::ffi::ECSWorld> *ptr) noexcept;
} // extern "C"

namespace rust {
inline namespace cxxbridge1 {
template <>
::gcs::ffi::ComponentData *Box<::gcs::ffi::ComponentData>::allocation::alloc() noexcept {
  return cxxbridge1$box$gcs$ffi$ComponentData$alloc();
}
template <>
void Box<::gcs::ffi::ComponentData>::allocation::dealloc(::gcs::ffi::ComponentData *ptr) noexcept {
  cxxbridge1$box$gcs$ffi$ComponentData$dealloc(ptr);
}
template <>
void Box<::gcs::ffi::ComponentData>::drop() noexcept {
  cxxbridge1$box$gcs$ffi$ComponentData$drop(this);
}
template <>
::gcs::ffi::EntityId *Box<::gcs::ffi::EntityId>::allocation::alloc() noexcept {
  return cxxbridge1$box$gcs$ffi$EntityId$alloc();
}
template <>
void Box<::gcs::ffi::EntityId>::allocation::dealloc(::gcs::ffi::EntityId *ptr) noexcept {
  cxxbridge1$box$gcs$ffi$EntityId$dealloc(ptr);
}
template <>
void Box<::gcs::ffi::EntityId>::drop() noexcept {
  cxxbridge1$box$gcs$ffi$EntityId$drop(this);
}
template <>
::gcs::ffi::ComponentInfo *Box<::gcs::ffi::ComponentInfo>::allocation::alloc() noexcept {
  return cxxbridge1$box$gcs$ffi$ComponentInfo$alloc();
}
template <>
void Box<::gcs::ffi::ComponentInfo>::allocation::dealloc(::gcs::ffi::ComponentInfo *ptr) noexcept {
  cxxbridge1$box$gcs$ffi$ComponentInfo$dealloc(ptr);
}
template <>
void Box<::gcs::ffi::ComponentInfo>::drop() noexcept {
  cxxbridge1$box$gcs$ffi$ComponentInfo$drop(this);
}
template <>
::gcs::ffi::ECSWorld *Box<::gcs::ffi::ECSWorld>::allocation::alloc() noexcept {
  return cxxbridge1$box$gcs$ffi$ECSWorld$alloc();
}
template <>
void Box<::gcs::ffi::ECSWorld>::allocation::dealloc(::gcs::ffi::ECSWorld *ptr) noexcept {
  cxxbridge1$box$gcs$ffi$ECSWorld$dealloc(ptr);
}
template <>
void Box<::gcs::ffi::ECSWorld>::drop() noexcept {
  cxxbridge1$box$gcs$ffi$ECSWorld$drop(this);
}
} // namespace cxxbridge1
} // namespace rust
