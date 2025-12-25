#ifndef __SCES_CMW_SCES_H__
#define __SCES_CMW_SCES_H__

/// @file sces.h
/// @brief SCES Common Definitions
/// @details This header file defines common types and return values used across the SCES
///          middleware and its components.
/// @author Khose-ie<khose-ie@outlook.com>

/// @brief Standard return value enumeration
/// @details Represents standard return values for SCES functions
typedef enum
{
    SCES_RET_OK                          = 0,
    SCES_RET_ERR_PARAM                   = 1,
    SCES_RET_ERR_BUSY                    = 2,
    SCES_RET_ERR_TIMEOUT                 = 3,
    SCES_RET_ERR_STACK_OVERFLOW          = 4,
    SCES_RET_ERR_PERMISSION              = 5,
    SCES_RET_ERR_NULL_REF                = 6,
    SCES_RET_ERR_MEM_ALLOC_FAILURE       = 16,
    SCES_RET_ERR_FORMAT_FAILURE          = 17,
    SCES_RET_ERR_LOW_LEVEL_FAILURE       = 18,
    SCES_RET_ERR_INSTANCE_CREATE_FAILURE = 32,
    SCES_RET_ERR_INSTANCE_NOT_FOUND      = 33,
    SCES_RET_ERR_INSTANCE_DUPLICATE      = 34,
    SCES_RET_ERR_INSTANCE_IN_USE         = 35,
    SCES_RET_ERR_INSTANCE_INVALID        = 36,
    SCES_RET_ERR_NOT_SUPPORT             = 48,
    SCES_RET_ERR_NOT_AVAILABLE           = 49,
    SCES_RET_ERR_UNKNOWN                 = 255
} scesRetVal_t;

#endif // __SCES_CMW_SCES_H__
