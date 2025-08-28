import React, {useState, useEffect} from 'react';
import {listen} from '@tauri-apps/api/event';
import {invoke} from '@tauri-apps/api/core';
import TaskForm from './components/TaskForm';
import TaskList from './components/TaskList';

const App = () => {
    const [tasks, setTasks] = useState([]);
    const [isLoading, setIsLoading] = useState(true);

    useEffect(() => {
        // 初始化任务列表
        const loadTasks = async () => {
            try {
                // 从Tauri后端获取初始任务数据
                const response = await invoke('plugin:task-queue|get_all_tasks');
                setTasks(response);
            } catch (error) {
                console.error('Failed to load tasks:', error);
            } finally {
                setIsLoading(false);
            }
        };

        loadTasks();

        // 监听任务状态更新事件
        let unlisten = null;
        const setupListener = async () => {
            unlisten = await listen('task_updated', (event) => {
                setTasks(prevTasks => {
                    const updatedTask = event.payload;
                    const taskIndex = prevTasks.findIndex(task => task.id === updatedTask.id);
                    if (taskIndex >= 0) {
                        const newTasks = [...prevTasks];
                        newTasks[taskIndex] = updatedTask;
                        return newTasks;
                    } else {
                        return [...prevTasks, updatedTask];
                    }
                });
            });
        };

        setupListener().then(_ => {
        });

        return () => {
            if (unlisten) {
                unlisten();
            }
        };
    }, []);

    const addTask = async (taskData) => {
        try {
            const newTask = await invoke('plugin:task-queue|add_task', {task: taskData});
            setTasks(prevTasks => [...prevTasks, newTask]);
            return true;
        } catch (error) {
            console.error('Failed to add task:', error);
            return false;
        }
    };

    const cancelTask = async (taskId) => {
        try {
            await invoke('cancel_task', {id: taskId});
            setTasks(prevTasks => prevTasks.map(task =>
                task.id === taskId ? {...task, status: 'cancelled'} : task
            ));
        } catch (error) {
            console.error('Failed to cancel task:', error);
        }
    };

    if (isLoading) {
        return <div className="loading">加载任务中...</div>;
    }

    return (
        <div className="app-container">
            <header>
                <h1>Tauri 任务队列插件示例</h1>
                <p>使用React重构的任务管理界面</p>
            </header>
            <main style={{padding: '20px'}}>
                <TaskForm onAddTask={addTask}/>
                <TaskList tasks={tasks} onCancelTask={cancelTask}/>
            </main>
        </div>
    );
};

export default App;