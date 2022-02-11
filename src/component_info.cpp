#include "component_info.h"


ComponentInfo::ComponentInfo() : componentInfo(gcs::ffi::create_component_info(0)){

}

void ComponentInfo::set_component_info(gcs::ffi::ComponentInfo* argComponentInfo) {
    componentInfo = rust::box<gcs::ffi::ComponentInfo>::from_raw(argComponentInfo);
}

gcs::ffi::ComponentInfo &ComponentInfo::getComponentInfo() {
    return componentInfo.operator*();
}

void ComponentInfo::_bind_methods() {
}
