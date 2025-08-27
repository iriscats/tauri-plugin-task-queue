// Tauri Task Queue 前端示例脚本

document.addEventListener('DOMContentLoaded', () => {
    // 获取DOM元素
    const addTaskBtn = document.getElementById('addTaskBtn');
    const refreshTasksBtn = document.getElementById('refreshTasksBtn');
    const clearCompletedBtn = document.getElementById('clearCompletedBtn');
    const taskTypeSelect = document.getElementById('taskType');
    const taskPrioritySelect = document.getElementById('taskPriority');
    const taskParamsTextarea = document.getElementById('taskParams');
    const tasksContainer = document.getElementById('tasksContainer');
    
    // 添加任务事件监听器
    addTaskBtn.addEventListener('click', addTask);
    
    // 刷新任务列表事件监听器
    refreshTasksBtn.addEventListener('click', loadTasks);
    
    // 清除已完成任务事件监听器
    clearCompletedBtn.addEventListener('click', clearCompletedTasks);
    
    // 页面加载时获取任务列表
    loadTasks();
    
    // 监听任务事件
    if (window.__TAURI__) {
        window.__TAURI__.event.listen('task_event', (event) => {
            console.log('收到任务事件:', event);
            const task = event.payload;
            updateTaskInList(task);
        });
    }
    
    // 添加任务函数
    async function addTask() {
        if (!window.__TAURI__) {
            alert('此功能仅在Tauri应用中可用');
            return;
        }
        
        try {
            const taskType = taskTypeSelect.value;
            const priority = parseInt(taskPrioritySelect.value);
            
            // 解析参数
            let params = {};
            if (taskParamsTextarea.value.trim()) {
                params = JSON.parse(taskParamsTextarea.value);
            }
            
            // 调用Tauri命令添加任务
            const taskId = await window.__TAURI__.invoke('enqueue_task', {
                taskType,
                params,
                priority
            });
            
            console.log('任务已添加，ID:', taskId);
            
            // 清空参数输入框
            taskParamsTextarea.value = '{}';
            
            // 重新加载任务列表
            loadTasks();
            
            // 显示成功消息
            showMessage('任务已成功添加', 'success');
        } catch (error) {
            console.error('添加任务失败:', error);
            showMessage('添加任务失败: ' + error.message, 'error');
        }
    }
    
    // 加载任务列表函数
    async function loadTasks() {
        if (!window.__TAURI__) {
            renderTasks([]);
            return;
        }
        
        try {
            const tasks = await window.__TAURI__.invoke('list_tasks');
            console.log('获取到任务列表:', tasks);
            renderTasks(tasks);
        } catch (error) {
            console.error('获取任务列表失败:', error);
            showMessage('获取任务列表失败: ' + error.message, 'error');
        }
    }
    
    // 清除已完成任务函数
    async function clearCompletedTasks() {
        // 这里可以调用一个Tauri命令来清除已完成的任务
        // 为了简化示例，我们只在前端过滤显示
        showMessage('此功能需要后端支持', 'info');
    }
    
    // 渲染任务列表
    function renderTasks(tasks) {
        if (!tasks || tasks.length === 0) {
            tasksContainer.innerHTML = '<p class="empty-state">暂无任务，请添加任务</p>';
            return;
        }
        
        // 按优先级排序（高优先级在前）
        tasks.sort((a, b) => a.priority - b.priority);
        
        const tasksHTML = tasks.map(task => {
            return `
                <div class="task-item" data-task-id="${task.id}">
                    <div class="task-header">
                        <div>
                            <span class="task-type">${task.task_type}</span>
                            <span class="task-id">ID: ${task.id.substring(0, 8)}...</span>
                        </div>
                        <div>
                            <span class="task-priority priority-${getPriorityClass(task.priority)}">${getPriorityText(task.priority)}</span>
                            <span class="task-status status-${getStatusClass(task.status)}">${getStatusText(task.status)}</span>
                        </div>
                    </div>
                    <div class="progress-container">
                        <div class="progress-bar">
                            <div class="progress-fill" style="width: ${task.progress}%"></div>
                        </div>
                        <div class="progress-text">${task.progress}%</div>
                    </div>
                    <div class="task-params">
                        ${JSON.stringify(task.params, null, 2)}
                    </div>
                </div>
            `;
        }).join('');
        
        tasksContainer.innerHTML = tasksHTML;
    }
    
    // 更新任务显示
    function updateTaskInList(task) {
        const taskElement = document.querySelector(`.task-item[data-task-id="${task.id}"]`);
        if (taskElement) {
            // 更新进度条
            const progressBar = taskElement.querySelector('.progress-fill');
            const progressText = taskElement.querySelector('.progress-text');
            if (progressBar && progressText) {
                progressBar.style.width = `${task.progress}%`;
                progressText.textContent = `${task.progress}%`;
            }
            
            // 更新状态
            const statusElement = taskElement.querySelector('.task-status');
            if (statusElement) {
                statusElement.className = `task-status status-${getStatusClass(task.status)}`;
                statusElement.textContent = getStatusText(task.status);
            }
        } else {
            // 如果任务元素不存在，重新加载列表
            loadTasks();
        }
    }
    
    // 获取优先级类名
    function getPriorityClass(priority) {
        switch (priority) {
            case 0: return 'high';
            case 1: return 'medium';
            case 2: return 'low';
            default: return 'medium';
        }
    }
    
    // 获取优先级文本
    function getPriorityText(priority) {
        switch (priority) {
            case 0: return '高';
            case 1: return '中';
            case 2: return '低';
            default: return '中';
        }
    }
    
    // 获取状态类名
    function getStatusClass(status) {
        if (typeof status === 'object') {
            return Object.keys(status)[0].toLowerCase();
        }
        return status.toLowerCase();
    }
    
    // 获取状态文本
    function getStatusText(status) {
        if (typeof status === 'object') {
            const statusKey = Object.keys(status)[0];
            switch (statusKey) {
                case 'Pending': return '待处理';
                case 'Running': return '进行中';
                case 'Completed': return '已完成';
                case 'Failed': return '失败';
                case 'Canceled': return '已取消';
                default: return statusKey;
            }
        }
        
        switch (status) {
            case 'Pending': return '待处理';
            case 'Running': return '进行中';
            case 'Completed': return '已完成';
            case 'Failed': return '失败';
            case 'Canceled': return '已取消';
            default: return status;
        }
    }
    
    // 显示消息
    function showMessage(message, type) {
        // 创建消息元素
        const messageElement = document.createElement('div');
        messageElement.className = `message message-${type}`;
        messageElement.textContent = message;
        
        // 添加样式
        messageElement.style.cssText = `
            position: fixed;
            top: 20px;
            right: 20px;
            padding: 15px 20px;
            border-radius: 4px;
            color: white;
            font-weight: 600;
            box-shadow: 0 4px 12px rgba(0,0,0,0.15);
            z-index: 1000;
            animation: slideIn 0.3s, fadeOut 0.5s 2.5s;
        `;
        
        // 根据类型设置背景色
        switch (type) {
            case 'success':
                messageElement.style.backgroundColor = '#2ecc71';
                break;
            case 'error':
                messageElement.style.backgroundColor = '#e74c3c';
                break;
            case 'info':
                messageElement.style.backgroundColor = '#3498db';
                break;
            default:
                messageElement.style.backgroundColor = '#95a5a6';
        }
        
        // 添加动画样式
        const style = document.createElement('style');
        style.textContent = `
            @keyframes slideIn {
                from { transform: translateX(100%); opacity: 0; }
                to { transform: translateX(0); opacity: 1; }
            }
            
            @keyframes fadeOut {
                from { opacity: 1; }
                to { opacity: 0; }
            }
        `;
        document.head.appendChild(style);
        
        // 添加到页面
        document.body.appendChild(messageElement);
        
        // 3秒后移除消息
        setTimeout(() => {
            messageElement.remove();
            style.remove();
        }, 3000);
    }
});