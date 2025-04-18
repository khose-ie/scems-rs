#include <cmsis_os2.h>
#include <stdint.h>
#include <string.h>
#include <assert.h>

#define MEM_POOL0_BLOCK_SIZE (64)
#define MEM_POOL0_BLOCK_NUM  (32 * 16)
#define MEM_POOL1_BLOCK_SIZE (256)
#define MEM_POOL1_BLOCK_NUM  (32 * 4)
#define MEM_POOL2_BLOCK_SIZE (512)
#define MEM_POOL2_BLOCK_NUM  (32 * 2)
#define MEM_POOL3_BLOCK_SIZE (1024)
#define MEM_POOL3_BLOCK_NUM  (16)
#define MEM_POOL4_BLOCK_SIZE (4096)
#define MEM_POOL4_BLOCK_NUM  (4)

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

void app_init(void)
{
    osMemoryPoolAttr_t mem_pool_attr;
    memset(&mem_pool_attr, 0, sizeof(osMemoryPoolAttr_t));

    mem_pool_attr.mp_mem  = mem_pool0_space;
    mem_pool_attr.mp_size = sizeof(mem_pool0_space);
    mem_pool0 = osMemoryPoolNew(MEM_POOL0_BLOCK_NUM, MEM_POOL0_BLOCK_SIZE, &mem_pool_attr);
    assert(mem_pool0 != NULL);

    mem_pool_attr.mp_mem  = mem_pool1_space;
    mem_pool_attr.mp_size = sizeof(mem_pool1_space);
    mem_pool1 = osMemoryPoolNew(MEM_POOL1_BLOCK_NUM, MEM_POOL1_BLOCK_SIZE, &mem_pool_attr);
    assert(mem_pool1 != NULL);

    mem_pool_attr.mp_mem  = mem_pool2_space;
    mem_pool_attr.mp_size = sizeof(mem_pool2_space);
    mem_pool2 = osMemoryPoolNew(MEM_POOL2_BLOCK_NUM, MEM_POOL2_BLOCK_SIZE, &mem_pool_attr);
    assert(mem_pool2 != NULL);

    mem_pool_attr.mp_mem  = mem_pool3_space;
    mem_pool_attr.mp_size = sizeof(mem_pool3_space);
    mem_pool3 = osMemoryPoolNew(MEM_POOL3_BLOCK_NUM, MEM_POOL3_BLOCK_SIZE, &mem_pool_attr);
    assert(mem_pool3 != NULL);

    mem_pool_attr.mp_mem  = mem_pool4_space;
    mem_pool_attr.mp_size = sizeof(mem_pool4_space);
    mem_pool4 = osMemoryPoolNew(MEM_POOL4_BLOCK_NUM, MEM_POOL4_BLOCK_SIZE, &mem_pool_attr);
    assert(mem_pool4 != NULL);
}

void app_main2(void)
{
    
}


