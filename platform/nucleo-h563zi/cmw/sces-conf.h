/// @file sces-conf.example.h
/// @brief SCES Configuration Header Example
/// @details This is an example configuration header file for the SCES OS abstraction layer.
///          It defines constants and macros that can be modified to customize the behavior
///          of the OS abstraction layer according to the specific requirements of the target
///          application and underlying operating system.
/// @note This file should be copied and renamed to sces-conf.h and modified as needed.
/// @author Khose-ie<khose-ie@outlook.com>
/// @date 2024-06-10

#ifndef __SCES_CONF_H__
#define __SCES_CONF_H__

//============================================================================================
/// @brief OS Stack Configuration
/// @details These constants and macros define the configuration for the OS stack byte pool.
//============================================================================================

/// @brief Size of the OS stack memory zone
/// @details This constant defines the size (in bytes) of the memory zone
///          allocated for the OS stack byte pool.
///          If you always use static stack allocation APIs, you can reduce this size
///          to save memory due to only management blocks are allocated from this pool.
/// @attention Modify this value according to your application's needs and this value
///            must be defined and aligned to uint32_t.
#define SCES_OS_STACK_SIZE (1024 * 60)

/// @brief Declaration of external the OS stack pool
/// @details This macro declares an external byte array that serves as the
///          memory pool for the OS stack byte pool.
///     If the SCES_OS_EX_STACK_POOL macro is not defined, a static byte pool will be created.
// #define SCES_OS_EX_STACK_POOL (os_stack)

/// @brief Declaration of the external OS stack memory zone
/// @details This macro declares an external byte array that serves as the
///          memory zone for the OS stack byte pool.
///     If the SCES_OS_EX_STACK_ZONE macro is not defined, a static array will be created.
/// @attention The size of this zone must be equal to @ref SCES_OS_STACK_SIZE.
// #define SCES_OS_EX_STACK_ZONE (os_stack_zone)


//============================================================================================
/// @brief OS Memory Pool Configuration
/// @details These constants define the block sizes and counts for the OS memory pool.
///          Modify these values according to your application's memory allocation patterns.
//============================================================================================

/// @brief Memory pool block size configuration #1
/// @details These constants define the block sizes and counts for the OS memory pool.
///          Modify these values according to your application's memory allocation patterns.
#define SCES_OS_MEM_POOL_BKSZ_1 (256)

/// @brief Memory pool block size configuration #2
/// @details These constants define the block sizes and counts for the OS memory pool.
///          Modify these values according to your application's memory allocation patterns.
#define SCES_OS_MEM_POOL_BKSZ_2 (512)

/// @brief Memory pool block size configuration #3
/// @details These constants define the block sizes and counts for the OS memory pool.
///          Modify these values according to your application's memory allocation patterns.
#define SCES_OS_MEM_POOL_BKSZ_3 (1024)

/// @brief Memory pool block size configuration #4
/// @details These constants define the block sizes and counts for the OS memory pool.
///          Modify these values according to your application's memory allocation patterns.
#define SCES_OS_MEM_POOL_BKSZ_4 (2048)

/// @brief Memory pool block count configuration #1
/// @details These constants define the block sizes and counts for the OS memory pool.
///          Modify these values according to your application's memory allocation patterns.
#define SCES_OS_MEM_POOL_BKCT_1 (32)

/// @brief Memory pool block count configuration #2
/// @details These constants define the block sizes and counts for the OS memory pool.
///          Modify these values according to your application's memory allocation patterns.
#define SCES_OS_MEM_POOL_BKCT_2 (16)

/// @brief Memory pool block count configuration #3
/// @details These constants define the block sizes and counts for the OS memory pool.
///          Modify these values according to your application's memory allocation patterns.
#define SCES_OS_MEM_POOL_BKCT_3 (8)

/// @brief Memory pool block count configuration #4
/// @details These constants define the block sizes and counts for the OS memory pool.
///          Modify these values according to your application's memory allocation patterns.
#define SCES_OS_MEM_POOL_BKCT_4 (4)

#endif // __SCES_CONF_H__