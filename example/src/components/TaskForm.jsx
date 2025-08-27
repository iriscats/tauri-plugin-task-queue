import React, { useState } from 'react';

const TaskForm = ({ onAddTask }) => {
  const [taskType, setTaskType] = useState('download');
  const [priority, setPriority] = useState('1');
  const [isSubmitting, setIsSubmitting] = useState(false);

  const handleSubmit = async (e) => {
    e.preventDefault();
    if (isSubmitting) return;

    setIsSubmitting(true);
    try {
      const taskData = {
        type: taskType,
        priority: parseInt(priority, 10),
        status: 'pending',
        progress: 0,
        created_at: new Date().toISOString()
      };

      const success = await onAddTask(taskData);
      if (success) {
        // 重置表单
        setTaskType('download');
        setPriority('1');
      }
    } finally {
      setIsSubmitting(false);
    }
  };

  return (
    <form onSubmit={handleSubmit} className="task-form">
      <h2>添加任务</h2>
      <div className="form-group">
        <label htmlFor="taskType">任务类型:</label>
        <select
          id="taskType"
          value={taskType}
          onChange={(e) => setTaskType(e.target.value)}
          disabled={isSubmitting}
        >
          <option value="download">下载任务</option>
          <option value="process">处理任务</option>
          <option value="upload">上传任务</option>
          <option value="compress">压缩任务</option>
        </select>
      </div>

      <div className="form-group">
        <label htmlFor="taskPriority">优先级:</label>
        <select
          id="taskPriority"
          value={priority}
          onChange={(e) => setPriority(e.target.value)}
          disabled={isSubmitting}
        >
          <option value="0">高</option>
          <option value="1">中</option>
          <option value="2">低</option>
        </select>
      </div>

      <button
        type="submit"
        className="btn-add-task"
        disabled={isSubmitting}
      >
        {isSubmitting ? '添加中...' : '添加任务'}
      </button>
    </form>
  );
};

export default TaskForm;