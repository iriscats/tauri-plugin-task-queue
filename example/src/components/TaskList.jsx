import React, { useState, useEffect } from 'react';
import { invoke } from '@tauri-apps/api/core';

const TaskList = ({ tasks: initialTasks, onCancelTask }) => {
  const [tasks, setTasks] = useState(initialTasks);

  useEffect(() => {
    setTasks(initialTasks);
  }, [initialTasks]);

  useEffect(() => {
    const interval = setInterval(async () => {
      try {
        const updatedTasks = await invoke('plugin:task-queue|get_all_tasks');
        setTasks(updatedTasks);
      } catch (error) {
        console.error('Failed to refresh tasks:', error);
      }
    }, 1000);

    return () => clearInterval(interval);
  }, []);
  if (tasks.length === 0) {
    return <div className="task-list-empty">暂无任务</div>;
  }

  // 根据优先级和创建时间排序任务
  const sortedTasks = [...tasks].sort((a, b) => {
    // 先按优先级排序（0:高, 1:中, 2:低）
    if (a.priority !== b.priority) {
      return a.priority - b.priority;
    }
    // 再按创建时间倒序排序
    return new Date(b.created_at) - new Date(a.created_at);
  });

  // 获取任务状态对应的样式类名
  const getStatusClass = (status) => {
    switch (status) {
      case 'pending':
        return 'status-pending';
      case 'processing':
        return 'status-processing';
      case 'completed':
        return 'status-completed';
      case 'failed':
        return 'status-failed';
      case 'cancelled':
        return 'status-cancelled';
      default:
        return '';
    }
  };

  // 获取任务类型显示名称
  const getTaskTypeName = (type) => {
    const typeMap = {
      'download': '下载任务',
      'process': '处理任务',
      'upload': '上传任务',
      'compress': '压缩任务'
    };
    return typeMap[type] || type;
  };

  // 获取优先级显示名称
  const getPriorityName = (priority) => {
    switch (priority) {
      case 0:
        return '高';
      case 1:
        return '中';
      case 2:
        return '低';
      default:
        return priority;
    }
  };

  return (
    <div className="task-list-container">
      <h2>任务列表</h2>
      <div className="task-list">
        {sortedTasks.map((task) => (
          <div key={task.id} className="task-item">
            <div className="task-header">
              <span className="task-type">{getTaskTypeName(task.type)}</span>
              <span className={`task-status ${getStatusClass(task.status)}`}>
                {task.status === 'pending' && '等待中'}
                {task.status === 'processing' && '处理中'}
                {task.status === 'completed' && '已完成'}
                {task.status === 'failed' && '失败'}
                {task.status === 'cancelled' && '已取消'}
              </span>
            </div>

            <div className="task-details">
              <div className="task-meta">
                <span className="task-priority">优先级: {getPriorityName(task.priority)}</span>
                <span className="task-created">创建时间: {new Date(task.created_at).toLocaleString()}</span>
              </div>

              {task.status === 'processing' && (
                <div className="task-progress">
                  <div className="progress-bar">
                    <div
                      className="progress-fill"
                      style={{ width: `${task.progress}%` }}
                    ></div>
                  </div>
                  <span className="progress-text">{Math.round(task.progress)}%</span>
                </div>
              )}

              {task.error && task.status === 'failed' && (
                <div className="task-error">
                  错误: {task.error}
                </div>
              )}

              {(task.status === 'pending' || task.status === 'processing') && (
                <button
                  className="btn-cancel-task"
                  onClick={() => onCancelTask(task.id)}
                >
                  取消任务
                </button>
              )}
            </div>
          </div>
        ))}
      </div>
    </div>
  );
};

export default TaskList;