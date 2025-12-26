
set(APP_INCS
    ${BASE}/sces-implements/sces-cmw/src-cwraps/inc 
    ${BASE}/platform/${PLATFORM}/cmw
    CACHE STRING ""
)

set(APP_SRCS
    ${BASE}/sces-wraps/sces-wrap-threadx/src/threadx.c
    CACHE STRING ""
)

set(APP_LIBS
    ${BASE}/${APP_PATH}/target/${ARCH}/${PROFILE}/lib${APP}.a
    CACHE STRING ""
)
