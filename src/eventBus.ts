/**
 * 一个简单的事件总线，用于组件间通信
 */
const eventBus = (() => {
  // 使用接口明确定义事件监听器类型
  interface EventListeners {
    [event: string]: Array<(data: any) => void>;
  }
  
  const listeners: EventListeners = {};
  
  return {
    /**
     * 监听事件
     * @param event 事件名称
     * @param callback 回调函数
     */
    on(event: string, callback: (data: any) => void) {
      if (!listeners[event]) {
        listeners[event] = [];
      }
      listeners[event].push(callback);
    },
    
    /**
     * 移除事件监听
     * @param event 事件名称
     * @param callback 回调函数
     */
    off(event: string, callback: (data: any) => void) {
      if (listeners[event]) {
        listeners[event] = listeners[event].filter(cb => cb !== callback);
      }
    },
    
    /**
     * 触发事件
     * @param event 事件名称
     * @param data 传递的数据
     */
    emit(event: string, data?: any) {
      if (listeners[event]) {
        listeners[event].forEach(callback => callback(data));
      }
    }
  };
})();

export default eventBus; 