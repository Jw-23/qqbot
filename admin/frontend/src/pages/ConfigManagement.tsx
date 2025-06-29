import React, { useState, useEffect } from 'react';
import { Form, Input, Button, Card, message, InputNumber, Switch, Space, Divider } from 'antd';
import { Config } from '../types';
import { configApi } from '../services/api';

const { TextArea } = Input;

const ConfigManagement: React.FC = () => {
  const [form] = Form.useForm();
  const [loading, setLoading] = useState(false);
  const [saveLoading, setSaveLoading] = useState(false);

  useEffect(() => {
    fetchConfig();
  }, []);

  const fetchConfig = async () => {
    setLoading(true);
    try {
      const response = await configApi.get();
      form.setFieldsValue(response.data);
    } catch (error) {
      message.error('获取配置失败');
    }
    setLoading(false);
  };

  const handleSubmit = async (values: Config) => {
    setSaveLoading(true);
    try {
      await configApi.update(values);
      message.success('配置更新成功');
    } catch (error) {
      message.error('配置更新失败');
    }
    setSaveLoading(false);
  };

  return (
    <div>
      <Card title="系统配置" loading={loading}>
        <Form
          form={form}
          layout="vertical"
          onFinish={handleSubmit}
          initialValues={{
            logging_level: 'INFO',
            cmd_suffix: '/',
            admins: [],
          }}
        >
          <Divider orientation="left">基础配置</Divider>
          
          <Form.Item
            name="logging_level"
            label="日志级别"
            rules={[{ required: true, message: '请输入日志级别' }]}
          >
            <Input placeholder="如: INFO, DEBUG, WARN, ERROR" />
          </Form.Item>

          <Form.Item
            name="cmd_suffix"
            label="命令后缀"
            rules={[{ required: true, message: '请输入命令后缀' }]}
          >
            <Input placeholder="如: /" />
          </Form.Item>

          <Form.Item
            name="admins"
            label="管理员列表"
            tooltip="每行一个管理员ID"
          >
            <TextArea rows={3} placeholder="请输入管理员ID，每行一个" />
          </Form.Item>

          <Divider orientation="left">缓存配置</Divider>
          
          <Form.Item name={['cache', 'cache_lifetime']} label="缓存生命周期">
            <Input placeholder="如: 10min" />
          </Form.Item>

          <Form.Item name={['cache', 'cache_capacity']} label="缓存容量">
            <InputNumber min={1} style={{ width: '100%' }} />
          </Form.Item>

          <Form.Item name={['cache', 'cache_idletime']} label="缓存空闲时间">
            <Input placeholder="如: 10min" />
          </Form.Item>

          <Form.Item name={['cache', 'conversation_capacity']} label="对话容量">
            <InputNumber min={1} style={{ width: '100%' }} />
          </Form.Item>

          <Form.Item name={['cache', 'max_conversation_history']} label="最大对话历史">
            <InputNumber min={1} style={{ width: '100%' }} />
          </Form.Item>

          <Form.Item name={['cache', 'conversation_timeout_minutes']} label="对话超时时间(分钟)">
            <InputNumber min={1} style={{ width: '100%' }} />
          </Form.Item>

          <Divider orientation="left">数据库配置</Divider>
          
          <Form.Item
            name={['database', 'url']}
            label="数据库连接URL"
            rules={[{ required: true, message: '请输入数据库连接URL' }]}
          >
            <Input placeholder="mysql://user:password@localhost/database" />
          </Form.Item>

          <Form.Item name={['database', 'max_connections']} label="最大连接数">
            <InputNumber min={1} style={{ width: '100%' }} />
          </Form.Item>

          <Form.Item name={['database', 'connect_timeout']} label="连接超时">
            <Input placeholder="如: 1min" />
          </Form.Item>

          <Form.Item name={['database', 'acquire_timeout']} label="获取连接超时">
            <Input placeholder="如: 20s" />
          </Form.Item>

          <Form.Item name={['database', 'idle_timeout']} label="空闲超时">
            <Input placeholder="如: 40s" />
          </Form.Item>

          <Form.Item name={['database', 'max_lifetime']} label="最大生命周期">
            <Input placeholder="如: 5min" />
          </Form.Item>

          <Form.Item name={['database', 'sqlx_logging']} label="SQL日志" valuePropName="checked">
            <Switch />
          </Form.Item>

          <Divider orientation="left">LLM配置</Divider>
          
          <Form.Item
            name={['llm', 'api_key']}
            label="API密钥"
            rules={[{ required: true, message: '请输入API密钥' }]}
          >
            <Input.Password placeholder="请输入API密钥" />
          </Form.Item>

          <Form.Item
            name={['llm', 'base_url']}
            label="API基础URL"
            rules={[{ required: true, message: '请输入API基础URL' }]}
          >
            <Input placeholder="https://api.siliconflow.cn/v1" />
          </Form.Item>

          <Form.Item
            name={['llm', 'model']}
            label="模型名称"
            rules={[{ required: true, message: '请输入模型名称' }]}
          >
            <Input placeholder="deepseek-ai/DeepSeek-R1-Distill-Qwen-14B" />
          </Form.Item>

          <Form.Item
            name={['llm', 'system_prompt']}
            label="系统提示"
            rules={[{ required: true, message: '请输入系统提示' }]}
          >
            <TextArea rows={3} placeholder="你是一个友好的QQ机器人助手..." />
          </Form.Item>

          <Form.Item name={['llm', 'temperature']} label="温度">
            <InputNumber min={0} max={2} step={0.1} style={{ width: '100%' }} />
          </Form.Item>

          <Form.Item name={['llm', 'max_tokens']} label="最大token数">
            <InputNumber min={1} style={{ width: '100%' }} />
          </Form.Item>

          <Form.Item name={['llm', 'top_p']} label="Top P">
            <InputNumber min={0} max={1} step={0.01} style={{ width: '100%' }} />
          </Form.Item>

          <Form.Item name={['llm', 'timeout_seconds']} label="超时时间(秒)">
            <InputNumber min={1} style={{ width: '100%' }} />
          </Form.Item>

          <Form.Item name={['llm', 'auto_capture_group_messages']} label="自动捕获群消息" valuePropName="checked">
            <Switch />
          </Form.Item>

          <Form.Item>
            <Space>
              <Button type="primary" htmlType="submit" loading={saveLoading}>
                保存配置
              </Button>
              <Button onClick={() => form.resetFields()}>
                重置
              </Button>
            </Space>
          </Form.Item>
        </Form>
      </Card>
    </div>
  );
};

export default ConfigManagement;
