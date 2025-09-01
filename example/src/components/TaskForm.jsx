import React, { useState } from 'react';

const TaskForm = ({ onAddTask }) => {
  const [taskType, setTaskType] = useState('download');
  const [priority, setPriority] = useState('1');
  const [params, setParams] = useState('{}');
  const [isSubmitting, setIsSubmitting] = useState(false);

  const handleSubmit = async (e) => {
    e.preventDefault();
    if (isSubmitting) return;

    setIsSubmitting(true);
    try {
        let paramsValue;
        try {
            paramsValue = JSON.parse(params);
        } catch (error) {
            console.error("Invalid JSON for params:", error);
            alert("参数必须是有效的JSON格式！");
            setIsSubmitting(false);
            return;
        }

      const success = await onAddTask(taskType, paramsValue, parseInt(priority, 10));
      if (success) {
        // 重置表单
        setTaskType('download');
        setPriority('1');
        setParams('{}');
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
        <label htmlFor="taskParams">参数 (JSON):</label>
        <textarea
            id="taskParams"
            value={params}
            onChange={(e) => setParams(e.target.value)}
            disabled={isSubmitting}
            rows="3"
        />
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