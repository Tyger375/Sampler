idf_component_register(SRCS

        settings/manager.cpp
        settings/manager.h

        settings/utils.cpp

        settings/group/group.cpp
        settings/group/group.h

        settings/component/component.cpp
        settings/component/component.h

        INCLUDE_DIRS .
        PRIV_REQUIRES
            littlefs
            arduinojson
)