#ifndef __SCES_CMW_SCES_OS_H__
#define __SCES_CMW_SCES_OS_H__

/// @file sces-os.h
/// @brief SCES OS Abstraction Layer
/// @details This header file defines the OS abstraction layer for SCES, providing
///          types and function prototypes for various OS services such as tasks,
///          events, message queues, memory pools, mutexes, semaphores, and timers.
///          The implementation of these functions should be provided in the corresponding
///          source file, tailored to the specific underlying OS.
/// @author Khose-ie<khose-ie@outlook.com>

#include <sces.h>
#include <stdint.h>

#define SCES_OS_WAIT_NO      0
#define SCES_OS_WAIT_FOREVER UINT32_MAX

#define SCES_EVENT_NONE (0x00000000U)
#define SCES_EVENT_ALL  (0xFFFFFFFFU)

/// @brief Event handle type
/// @details Opaque handle representing an event object
typedef void* scesEventHandle_t;

/// @brief Message queue handle type
/// @details Opaque handle representing a message queue object
typedef void* scesMessageQueueHandle_t;

/// @brief Memory pool handle type
/// @details Opaque handle representing a memory pool object
typedef void* scesMemPoolHandle_t;

/// @brief Mutex handle type
/// @details Opaque handle representing a mutex object
typedef void* scesMutexHandle_t;

/// @brief Semaphore handle type
/// @details Opaque handle representing a semaphore object
typedef void* scesSemaphoreHandle_t;

/// @brief Task handle type
/// @details Opaque handle representing a task control block
typedef void* scesTaskHandle_t;

/// @brief Timer handle type
/// @details Opaque handle representing a timer object
typedef void* scesTimerHandle_t;

/// @brief OS state enumeration
/// @details Represents the various states the OS can be in
typedef enum
{
    SCES_OS_STATE_RUNNING      = 0,
    SCES_OS_STATE_INITIALIZING = 1,
    SCES_OS_STATE_BLOCKED      = 2,
    SCES_OS_STATE_SUSPENDED    = 3,
    SCES_OS_STATE_LOCKED       = 4,
    SCES_OS_STATE_TERMINATED   = 5,
    SCES_OS_STATE_ERR_INIT_MEM = 6,
    SCES_OS_STATE_UNKNOWN_ERR  = UINT32_MAX
} scesOsState_t;

/// @brief Task state enumeration
/// @details Represents the various states a task can be in
typedef enum
{
    SCES_TASK_STATE_INACTIVE   = 0,
    SCES_TASK_STATE_READY      = 1,
    SCES_TASK_STATE_RUNNING    = 2,
    SCES_TASK_STATE_BLOCKED    = 3,
    SCES_TASK_STATE_TERMINATED = 4,
    SCES_TASK_STATE_ERROR      = 5,
    SCES_TASK_STATE_UNKNOWN    = UINT32_MAX
} scesTaskState_t;

/// @brief Task priority enumeration
/// @details Represents the priority levels for tasks
typedef enum
{
    SCES_TASK_PRIORITY_NONE      = 0,
    SCES_TASK_PRIORITY_IDLE      = 1,
    SCES_TASK_PRIORITY_BASE      = 2,
    SCES_TASK_PRIORITY_LOW       = 3,
    SCES_TASK_PRIORITY_NORMAL    = 4,
    SCES_TASK_PRIORITY_HIGH      = 5,
    SCES_TASK_PRIORITY_PRIVILEGE = 6,
    SCES_TASK_PRIORITY_REALTIME  = 7,
    SCES_TASK_PRIORITY_MAX       = SCES_TASK_PRIORITY_REALTIME
} scesTaskPriority_t;

/// @brief Timer state enumeration
/// @details Represents the various states a timer can be in
typedef enum
{
    SCES_TIMER_STATE_IDLE    = 0,
    SCES_TIMER_STATE_ACTIVE  = 1,
    SCES_TIMER_STATE_EXPIRED = 2,
    SCES_TIMER_STATE_DELETED = 3,
    SCES_TIMER_STATE_ERROR   = 4,
    SCES_TIMER_STATE_UNKNOWN = UINT32_MAX
} scesTimerState_t;

/// @brief Get the current OS state
/// @return Current OS state
scesOsState_t sces_os_state(void);

/// @brief Get the current system tick count
/// @return Current system tick count in milliseconds
uint32_t sces_os_tick_count(void);

/// @brief  Get the number of tasks currently in the system
/// @return Number of tasks in the system
uint32_t sces_os_task_count(void);

/// @brief  Suspend the task scheduler
/// @return Previous scheduler state to be used for resuming
uint32_t sces_os_suspend_schedule(void);

/// @brief  Resume the task scheduler
/// @param previous_state Previous scheduler state returned by @ref sces_os_suspend_schedule
void sces_os_resume_schedule(uint32_t previous_state);

/// @brief  Get the currently running task
/// @return Handle to the currently running task
scesTaskHandle_t sces_os_current_task(void);

/// @brief  Yield the processor from the current task
/// @details This function allows the current task to yield the processor, allowing other tasks to
/// run.
void sces_os_yield(void);

/// @brief  Delay the current task for a specified time
/// @details This function puts the current task into a blocked state for the specified number of
/// system ticks.
/// @param ticks Number of system ticks to delay
void sces_os_delay(uint32_t ticks);

/// @brief  Delay the current task until a specified time
/// @details This function delays the current task until the specified time increment has passed
/// since the previous wake time.
/// @param ticks     Time increment in system ticks
void sces_os_delay_until(uint32_t ticks);

/// @brief  Exit the current task
/// @details This function terminates the execution of the current task.
void sces_os_exit_task(void);

/// @brief  Exit the current task created with static stack allocation
/// @details This function terminates the execution of the current task created with static stack
void sces_os_exit_task_static(void);

/// @brief  Create a new event object
/// @details This function creates a new event object with the specified name.
/// @param name Name of the event object
/// @return Handle to the created event object or NULL on failure
scesEventHandle_t sces_event_create(const char* name);

/// @brief  Delete an event object
/// @details This function deletes the specified event object and frees its resources.
/// @param event Handle to the event object to be deleted
void sces_event_delete(scesEventHandle_t event);

/// @brief  Get the name of an event object
/// @details This function retrieves the name of the specified event object.
/// @param event Handle to the event object
/// @return Pointer to the event object's name string
const char* sces_event_name(scesEventHandle_t event);

/// @brief  Get the current state of an event object
/// @details This function retrieves the current state (flags) of the specified event object.
/// @param event Handle to the event object
/// @return Current state (flags) of the event object
uint32_t sces_event_state(scesEventHandle_t event);

/// @brief  Put event flags
/// @details This function puts the specified flags in the event object.
/// @param event Handle to the event object
/// @param flags Flags to be putted
/// @return SCES_RET_OK on success, error code otherwise
scesRetVal_t sces_event_put(scesEventHandle_t event, uint32_t flags);

/// @brief  Wait for event flags
/// @details This function waits for the specified flags to be set in the event object.
/// @param event        Handle to the event object
/// @param events_value Flags to wait for
/// @param timeout      Timeout in milliseconds to wait (0 for no wait, SCES_OS_WAIT_FOREVER for
/// infinite wait)
/// @return SCES_RET_OK on success, error code otherwise
scesRetVal_t sces_event_wait(scesEventHandle_t event, uint32_t events_value, uint32_t timeout);

/// @brief  Clear event flags
/// @details This function clears the specified flags in the event object.
/// @param events_value Handle to the event object
/// @param flags Flags to be cleared
void sces_event_clear(scesEventHandle_t event, uint32_t events_value);

/// @brief  Wait for event flags and clear them
/// @details This function waits for the specified flags to be set in the event object and clears
/// them upon wakeup.
/// @param event             Handle to the event object
/// @param events_value      Flags to wait for
/// @param out_events_value  Pointer to store the flags that caused the wakeup
/// @param timeout           Timeout in milliseconds to wait (0 for no wait, SCES_OS_WAIT_FOREVER
/// for infinite wait)
/// @return SCES_RET_OK on success, error code otherwise
scesRetVal_t sces_event_wait_and_clear(scesEventHandle_t event, uint32_t events_value,
                                       uint32_t* out_events_value, uint32_t timeout);

/// @brief  Create a new message queue
/// @details This function creates a new message queue with the specified parameters.
/// @param name          Name of the message queue
/// @param message_size  Size of each message in bytes
/// @param message_count Maximum number of messages the queue can hold
/// @return Handle to the created message queue or NULL on failure
scesMessageQueueHandle_t sces_mq_create(const char* name, uint32_t message_size,
                                        uint32_t message_count);

/// @brief  Create a new message queue with static buffer
/// @details This function creates a new message queue with the specified parameters,
///     using a statically allocated message buffer.
/// @param name           Name of the message queue
/// @param message_buffer Pointer to the statically allocated message buffer
/// @param message_size   Size of each message in bytes
/// @param message_count  Maximum number of messages the queue can hold
/// @return Handle to the created message queue
scesMessageQueueHandle_t sces_mq_create_static(const char* name, uint8_t* message_buffer,
                                               uint32_t message_size, uint32_t message_count);

/// @brief  Delete a message queue
/// @details This function deletes the specified message queue and frees its resources.
/// @param queue Handle to the message queue to be deleted
void sces_mq_delete(scesMessageQueueHandle_t queue);

/// @brief  Delete a message queue created with static buffer
/// @details This function deletes the specified message queue created with a static buffer.
/// @param queue Handle to the message queue to be deleted
void sces_mq_delete_static(scesMessageQueueHandle_t queue);

/// @brief  Get the name of a message queue
/// @param queue Handle to the message queue
/// @return Pointer to the message queue's name string
const char* sces_mq_name(scesMessageQueueHandle_t queue);

/// @brief  Get the size of a message in the queue
/// @param queue Handle to the message queue
/// @return Size of each message in bytes
uint32_t sces_mq_message_size(scesMessageQueueHandle_t queue);

/// @brief  Get the number of messages currently in the queue
/// @param queue Handle to the message queue
/// @return Number of messages currently in the queue
uint32_t sces_mq_message_count(scesMessageQueueHandle_t queue);

/// @brief  Get the maximum number of messages the queue can hold
/// @param queue Handle to the message queue
/// @return Maximum number of messages the queue can hold
uint32_t sces_mq_max_message_count(scesMessageQueueHandle_t queue);

/// @brief  Receive a message from the queue
/// @details This function receives a message from the specified message queue.
/// @param queue    Handle to the message queue
/// @param message  Pointer to the buffer to store the received message
/// @param timeout  Timeout in milliseconds to wait for a message (0 for no wait, UINT32_MAX for
/// infinite wait)
/// @return SCES_RET_OK on success, error code otherwise
scesRetVal_t sces_mq_send(scesMessageQueueHandle_t queue, const void* message, uint32_t timeout);

/// @brief  Receive a message from the queue
/// @details This function receives a message from the specified message queue.
/// @param queue    Handle to the message queue
/// @param message  Pointer to the buffer to store the received message
/// @param timeout  Timeout in milliseconds to wait for a message (0 for no wait,
/// SCES_OS_WAIT_FOREVER for infinite wait)
/// @return SCES_RET_OK on success, error code otherwise
scesRetVal_t sces_mq_receive(scesMessageQueueHandle_t queue, void* message, uint32_t timeout);

/// @brief  Clear all messages from the queue
/// @details This function clears all messages currently in the specified message queue.
/// @param queue Handle to the message queue
void sces_mq_clear(scesMessageQueueHandle_t queue);

/// @brief  Create a new memory pool
/// @details This function creates a new memory pool with the specified parameters.
/// @param name        Name of the memory pool
/// @param block_size  Size of each memory block in bytes
/// @param block_count Number of memory blocks in the pool
/// @return Handle to the created memory pool or NULL on failure
scesMemPoolHandle_t sces_mem_pool_create(const char* name, uint32_t block_size,
                                         uint32_t block_count);

/// @brief  Create a new memory pool with static buffer
/// @details This function creates a new memory pool with the specified parameters,
///     using a statically allocated pool buffer.
/// @param name        Name of the memory pool
/// @param pool_buffer Pointer to the statically allocated pool buffer
/// @param block_size  Size of each memory block in bytes
/// @param block_count Number of memory blocks in the pool
/// @return Handle to the created memory pool or NULL on failure
scesMemPoolHandle_t sces_mem_pool_create_static(const char* name, uint8_t* pool_buffer,
                                                uint32_t block_size, uint32_t block_count);

/// @brief  Delete a memory pool
/// @details This function deletes the specified memory pool and frees its resources.
/// @param pool Handle to the memory pool to be deleted
void sces_mem_pool_delete(scesMemPoolHandle_t pool);

/// @brief  Delete a memory pool created with static buffer
/// @details This function deletes the specified memory pool created with a static buffer.
/// @param pool Handle to the memory pool to be deleted
void sces_mem_pool_delete_static(scesMemPoolHandle_t pool);

/// @brief  Get the name of a memory pool
/// @param pool Handle to the memory pool
/// @return Pointer to the memory pool's name string
const char* sces_mem_pool_name(scesMemPoolHandle_t pool);

/// @brief  Get the size of each memory block in the pool
/// @param pool Handle to the memory pool
/// @return Size of each memory block in bytes
uint32_t sces_mem_pool_block_size(scesMemPoolHandle_t pool);

/// @brief  Get the number of used memory blocks in the pool
/// @param pool Handle to the memory pool
/// @return Number of memory blocks in the pool
uint32_t sces_mem_pool_block_count(scesMemPoolHandle_t pool);

/// @brief  Get the maximum number of memory blocks in the pool
/// @param pool Handle to the memory pool
/// @return Maximum number of memory blocks in the pool
uint32_t sces_mem_pool_max_block_count(scesMemPoolHandle_t pool);

/// @brief  Allocate a memory block from the pool
/// @details This function allocates a memory block from the specified memory pool.
/// @param pool    Handle to the memory pool
/// @param timeout Timeout in milliseconds to wait for a memory block (0 for no wait,
/// SCES_OS_WAIT_FOREVER for infinite wait)
/// @return Pointer to the allocated memory block, or NULL on failure
void* sces_mem_pool_alloc(scesMemPoolHandle_t pool, uint32_t timeout);

/// @brief  Free a memory block back to the pool
/// @details This function frees a previously allocated memory block back to the specified memory
/// pool.
/// @param pool  Handle to the memory pool
/// @param block Pointer to the memory block to be freed
void sces_mem_pool_free(scesMemPoolHandle_t pool, void* block);

/// @brief  Create a new mutex
/// @details This function creates a new mutex with the specified name.
/// @param name Name of the mutex
/// @return Handle to the created mutex or NULL on failure
scesMutexHandle_t sces_mutex_create(const char* name);

/// @brief  Delete a mutex
/// @details This function deletes the specified mutex and frees its resources.
/// @param mutex Handle to the mutex to be deleted
void sces_mutex_delete(scesMutexHandle_t mutex);

/// @brief  Get the name of a mutex
/// @param mutex Handle to the mutex
/// @return Pointer to the mutex's name string
const char* sces_mutex_name(scesMutexHandle_t mutex);

/// @brief  Get the owner of a mutex
/// @details This function retrieves the handle of the task that currently owns the specified
/// mutex.
/// @param mutex Handle to the mutex
/// @return Handle to the task that currently owns the mutex
scesTaskHandle_t sces_mutex_owner(scesMutexHandle_t mutex);

/// @brief  Lock a mutex
/// @details This function locks the specified mutex, blocking the calling task if necessary.
/// @param mutex   Handle to the mutex
/// @param timeout Timeout in milliseconds to wait for the mutex (0 for no wait,
/// SCES_OS_WAIT_FOREVER for infinite wait)
/// @return SCES_RET_OK on success, error code otherwise
scesRetVal_t sces_mutex_lock(scesMutexHandle_t mutex, uint32_t timeout);

/// @brief  Unlock a mutex
/// @details This function unlocks the specified mutex.
/// @param mutex Handle to the mutex
/// @return SCES_RET_OK on success, error code otherwise
scesRetVal_t sces_mutex_unlock(scesMutexHandle_t mutex);

/// @brief  Create a new semaphore
/// @details This function creates a new semaphore with the specified parameters.
/// @param name          Name of the semaphore
/// @param initial_count Initial count of the semaphore
/// @param max_count     Maximum count of the semaphore
/// @return Handle to the created semaphore, or NULL on failure
scesSemaphoreHandle_t sces_semaphore_create(const char* name, uint32_t initial_count,
                                            uint32_t max_count);

/// @brief  Delete a semaphore
/// @details This function deletes the specified semaphore and frees its resources.
/// @param semaphore Handle to the semaphore to be deleted
void sces_semaphore_delete(scesSemaphoreHandle_t semaphore);

/// @brief  Get the name of a semaphore
/// @param semaphore Handle to the semaphore
/// @return Pointer to the semaphore's name string
const char* sces_semaphore_name(scesSemaphoreHandle_t semaphore);

/// @brief  Get the current count of a semaphore
/// @param semaphore Handle to the semaphore
/// @return Current count of the semaphore
uint32_t sces_semaphore_count(scesSemaphoreHandle_t semaphore);

/// @brief  Take (decrement) a semaphore
/// @details This function takes (decrements) the specified semaphore, blocking the calling task
/// if necessary.
/// @param semaphore Handle to the semaphore
/// @param timeout   Timeout in milliseconds to wait for the semaphore (0 for no wait,
/// SCES_OS_WAIT_FOREVER for infinite wait)
/// @return SCES_RET_OK on success, error code otherwise
scesRetVal_t sces_semaphore_take(scesSemaphoreHandle_t semaphore, uint32_t timeout);

/// @brief  Release (increment) a semaphore
/// @details This function releases (increments) the specified semaphore.
/// @param semaphore Handle to the semaphore
/// @return SCES_RET_OK on success, error code otherwise
scesRetVal_t sces_semaphore_release(scesSemaphoreHandle_t semaphore);

/// @brief  Create a new task
/// @details This function creates a new task with the specified parameters.
///     The task will start executing the provided main function with the given argument.
/// @param name        Name of the task
/// @param main        Pointer to the task's main function
/// @param arg         Argument to be passed to the task's main function
/// @param stack_size  Size of the task's stack in bytes
/// @param priority    Priority of the task (use SCES_TASK_PRIORITY_* constants)
/// @return Handle to the created task, or NULL on failure
scesTaskHandle_t sces_task_create(const char* name, void (*main)(void*), void* arg,
                                  uint32_t stack_size, scesTaskPriority_t priority);

/// @brief  Create a new task with static stack allocation
/// @details This function creates a new task with the specified parameters.
///     The task will start executing the provided main function with the given argument.
/// @param name        Name of the task
/// @param main        Pointer to the task's main function
/// @param arg         Argument to be passed to the task's main function
/// @param stack       Pointer to the static stack buffer
/// @param stack_size  Size of the task's stack in bytes
/// @param priority    Priority of the task (use SCES_TASK_PRIORITY_* constants)
/// @return Handle to the created task, or NULL on failure
scesTaskHandle_t sces_task_create_static(const char* name, void (*main)(void*), void* arg,
                                         uint8_t* stack, uint32_t stack_size,
                                         scesTaskPriority_t priority);

/// @brief  Delete a task
/// @details This function deletes the specified task and frees its resources.
/// @param task Handle to the task to be deleted
void sces_task_delete(scesTaskHandle_t task);

/// @brief  Delete a task created with static stack allocation
/// @details This function deletes the specified task created with static stack allocation.
/// @param task Handle to the task to be deleted
void sces_task_delete_static(scesTaskHandle_t task);

/// @brief  Get the name of a task
/// @param task Handle to the task control block
/// @return Pointer to the task's name string
const char* sces_task_name(scesTaskHandle_t task);

/// @brief  Get the current state of a task
/// @param task Handle to the task control block
/// @return Current state of the task
uint32_t sces_task_stack_size(scesTaskHandle_t task);

/// @brief  Get the priority of a task
/// @param task Handle to the task control block
/// @return Priority of the task
scesTaskPriority_t sces_task_priority(scesTaskHandle_t task);

/// @brief  Get the current state of a task
/// @param task Handle to the task control block
/// @return Current state of the task
scesTaskState_t sces_task_state(scesTaskHandle_t task);

/// @brief  Set the priority of a task
/// @param task     Handle to the task control block
/// @param priority New priority to be set for the task
/// @return SCES_RET_OK on success, error code otherwise
scesRetVal_t sces_task_set_priority(scesTaskHandle_t task, scesTaskPriority_t priority);

/// @brief  Suspend a task
/// @param task Handle to the task control block to be suspended
void sces_task_suspend(scesTaskHandle_t task);

/// @brief  Resume a suspended task
/// @param task Handle to the task control block to be resumed
void sces_task_resume(scesTaskHandle_t task);

// /// @brief  Join a task
// /// @details This function waits for the specified task to complete its execution.
// /// @param task Handle to the task to be joined
// /// @return SCES_RET_OK on success, error code otherwise
// scesRetVal_t sces_task_join(scesTaskHandle_t task);

// /// @brief  Detach a task
// /// @details This function detaches the specified task, allowing it to run independently.
// /// @param task Handle to the task to be detached
// /// @return SCES_RET_OK on success, error code otherwise
// scesRetVal_t sces_task_detach(scesTaskHandle_t task);

/// @brief  Create a new timer
/// @details This function creates a new timer with the specified parameters.
/// @param name        Name of the timer
/// @param callback    Pointer to the timer's callback function
/// @param arg         Argument to be passed to the timer's callback function
/// @return Handle to the created timer, or NULL on failure
scesTimerHandle_t sces_timer_create_once(const char* name, void (*callback)(void*), void* arg);

/// @brief  Create a new periodic timer
/// @details This function creates a new periodic timer with the specified parameters.
/// @param name        Name of the timer
/// @param callback    Pointer to the timer's callback function
/// @param arg         Argument to be passed to the timer's callback function
scesTimerHandle_t sces_timer_create_periodic(const char* name, void (*callback)(void*), void* arg);

/// @brief  Delete a timer
/// @details This function deletes the specified timer and frees its resources.
/// @param timer Handle to the timer to be deleted
void sces_timer_delete(scesTimerHandle_t timer);

/// @brief  Get the name of a timer
/// @param timer Handle to the timer
/// @return Pointer to the timer's name string
const char* sces_timer_name(scesTimerHandle_t timer);

/// @brief  Get the current state of a timer
/// @param timer Handle to the timer
/// @return Current state of the timer
scesTimerState_t sces_timer_state(scesTimerHandle_t timer);

/// @brief  Start a timer
/// @details This function starts the specified timer with the given timeout.
/// @param timer   Handle to the timer
/// @param timeout Timeout in OS count for the timer
/// @return SCES_RET_OK on success, error code otherwise
scesRetVal_t sces_timer_start(scesTimerHandle_t timer, uint32_t timeout);

/// @brief  Stop a timer
/// @details This function stops the specified timer.
/// @param timer Handle to the timer
void sces_timer_stop(scesTimerHandle_t timer);

#endif // __SCES_CMW_SCES_OS_H__
