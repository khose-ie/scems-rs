#include <cmsis_os2.h>
#include <stdint.h>
#include <string.h>
#include <assert.h>
#include <FreeRTOS.h>

#include "main.h"
#include "tim.h"
#include "challen-v2-f429.main.h"

__attribute__((section(".ccmram"))) uint8_t ucHeap[configTOTAL_HEAP_SIZE];

extern void app_main(void);

void AppMain(void)
{
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
