#include <cmsis_os2.h>
#include <stdint.h>
#include <string.h>
#include <assert.h>
#include <FreeRTOS.h>

#include "main.h"
#include "tim.h"
#include "challen-v2-f429.main.h"

#ifndef MEM_POOL0_BLOCK_SIZE
#define MEM_POOL0_BLOCK_SIZE (0)
#endif /* MEM_POOL0_BLOCK_SIZE */

#ifndef MEM_POOL0_BLOCK_NUM
#define MEM_POOL0_BLOCK_NUM  (0)
#endif /* MEM_POOL0_BLOCK_SIZE */

#ifndef MEM_POOL1_BLOCK_SIZE
#define MEM_POOL1_BLOCK_SIZE (0)
#endif /* MEM_POOL1_BLOCK_SIZE */

#ifndef MEM_POOL1_BLOCK_NUM
#define MEM_POOL1_BLOCK_NUM  (0)
#endif /* MEM_POOL1_BLOCK_NUM */

#ifndef MEM_POOL2_BLOCK_SIZE
#define MEM_POOL2_BLOCK_SIZE (0)
#endif /* MEM_POOL2_BLOCK_SIZE */

#ifndef MEM_POOL2_BLOCK_NUM
#define MEM_POOL2_BLOCK_NUM  (0)
#endif /* MEM_POOL2_BLOCK_NUM */

#ifndef MEM_POOL3_BLOCK_SIZE
#define MEM_POOL3_BLOCK_SIZE (0)
#endif /* MEM_POOL3_BLOCK_SIZE */

#ifndef MEM_POOL3_BLOCK_NUM
#define MEM_POOL3_BLOCK_NUM  (0)
#endif /* MEM_POOL3_BLOCK_NUM */

#ifndef MEM_POOL4_BLOCK_SIZE
#define MEM_POOL4_BLOCK_SIZE (0)
#endif /* MEM_POOL4_BLOCK_SIZE */

#ifndef MEM_POOL4_BLOCK_NUM
#define MEM_POOL4_BLOCK_NUM  (0)
#endif /* MEM_POOL4_BLOCK_NUM */

__attribute__((section(".ccmram"))) uint8_t ucHeap[configTOTAL_HEAP_SIZE];

extern osMemoryPoolId_t mem_pool0;
extern const uint32_t mem_pool0_block_size;
extern osMemoryPoolId_t mem_pool1;
extern const uint32_t mem_pool1_block_size;
extern osMemoryPoolId_t mem_pool2;
extern const uint32_t mem_pool2_block_size;
extern osMemoryPoolId_t mem_pool3;
extern const uint32_t mem_pool3_block_size;
extern osMemoryPoolId_t mem_pool4;
extern const uint32_t mem_pool4_block_size;

const uint32_t mem_pool0_block_size = MEM_POOL0_BLOCK_SIZE;
const uint32_t mem_pool1_block_size = MEM_POOL1_BLOCK_SIZE;
const uint32_t mem_pool2_block_size = MEM_POOL2_BLOCK_SIZE;
const uint32_t mem_pool3_block_size = MEM_POOL3_BLOCK_SIZE;
const uint32_t mem_pool4_block_size = MEM_POOL4_BLOCK_SIZE;

osMemoryPoolId_t mem_pool0;
static uint8_t mem_pool0_space[MEM_POOL0_BLOCK_SIZE * MEM_POOL0_BLOCK_NUM];

osMemoryPoolId_t mem_pool1;
static uint8_t mem_pool1_space[MEM_POOL1_BLOCK_SIZE * MEM_POOL1_BLOCK_NUM];

osMemoryPoolId_t mem_pool2;
static uint8_t mem_pool2_space[MEM_POOL2_BLOCK_SIZE * MEM_POOL2_BLOCK_NUM];

osMemoryPoolId_t mem_pool3;
static uint8_t mem_pool3_space[MEM_POOL3_BLOCK_SIZE * MEM_POOL3_BLOCK_NUM];

osMemoryPoolId_t mem_pool4;
static uint8_t mem_pool4_space[MEM_POOL4_BLOCK_SIZE * MEM_POOL4_BLOCK_NUM];

extern void app_main(void);

void AppMain(void)
{
    osMemoryPoolAttr_t mem_pool_attr;
    memset(&mem_pool_attr, 0, sizeof(osMemoryPoolAttr_t));

#if MEM_POOL0_BLOCK_SIZE != 0
    mem_pool_attr.mp_mem  = mem_pool0_space;
    mem_pool_attr.mp_size = sizeof(mem_pool0_space);
    mem_pool0 = osMemoryPoolNew(MEM_POOL0_BLOCK_NUM, MEM_POOL0_BLOCK_SIZE, &mem_pool_attr);
    assert(mem_pool0 != NULL);
#endif /* MEM_POOL0_BLOCK_SIZE */

#if MEM_POOL1_BLOCK_SIZE != 0
    mem_pool_attr.mp_mem  = mem_pool1_space;
    mem_pool_attr.mp_size = sizeof(mem_pool1_space);
    mem_pool1 = osMemoryPoolNew(MEM_POOL1_BLOCK_NUM, MEM_POOL1_BLOCK_SIZE, &mem_pool_attr);
    assert(mem_pool1 != NULL);
#endif /* MEM_POOL1_BLOCK_SIZE */

#if MEM_POOL2_BLOCK_SIZE != 0
    mem_pool_attr.mp_mem  = mem_pool2_space;
    mem_pool_attr.mp_size = sizeof(mem_pool2_space);
    mem_pool2 = osMemoryPoolNew(MEM_POOL2_BLOCK_NUM, MEM_POOL2_BLOCK_SIZE, &mem_pool_attr);
    assert(mem_pool2 != NULL);
#endif /* MEM_POOL2_BLOCK_SIZE */

#if MEM_POOL3_BLOCK_SIZE != 0
    mem_pool_attr.mp_mem  = mem_pool3_space;
    mem_pool_attr.mp_size = sizeof(mem_pool3_space);
    mem_pool3 = osMemoryPoolNew(MEM_POOL3_BLOCK_NUM, MEM_POOL3_BLOCK_SIZE, &mem_pool_attr);
    assert(mem_pool3 != NULL);
#endif /* MEM_POOL3_BLOCK_SIZE */

#if MEM_POOL4_BLOCK_SIZE != 0
    mem_pool_attr.mp_mem  = mem_pool4_space;
    mem_pool_attr.mp_size = sizeof(mem_pool4_space);
    mem_pool4 = osMemoryPoolNew(MEM_POOL4_BLOCK_NUM, MEM_POOL4_BLOCK_SIZE, &mem_pool_attr);
    assert(mem_pool4 != NULL);
#endif /* MEM_POOL4_BLOCK_SIZE */

#if defined(APP)
    app_main();
#endif /* APP */
}

void AppTaskMain(void *argument)
{
    (void)argument;
    while(1) { osDelay(0x0FFFFFFF); }
}

void configureTimerForRunTimeStats(void)
{
    HAL_TIM_Base_Start(&RTOS_CHECK_TIM);
}

unsigned long getRunTimeCounterValue(void)
{
    return __HAL_TIM_GET_COUNTER(&RTOS_CHECK_TIM);
}
