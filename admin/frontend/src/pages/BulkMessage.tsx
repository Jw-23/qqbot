import React, { useState, useEffect } from 'react';
import { Card, Form, Input, Button, Select, message, Space, List, Tag } from 'antd';
import { SendOutlined, UserOutlined } from '@ant-design/icons';
import { Student } from '../types';
import { studentApi } from '../services/api';

const { TextArea } = Input;
const { Option } = Select;

const BulkMessage: React.FC = () => {
  const [form] = Form.useForm();
  const [students, setStudents] = useState<Student[]>([]);
  const [selectedStudents, setSelectedStudents] = useState<number[]>([]);
  const [loading, setLoading] = useState(false);
  const [sendLoading, setSendLoading] = useState(false);

  useEffect(() => {
    fetchStudents();
  }, []);

  const fetchStudents = async () => {
    setLoading(true);
    try {
      const response = await studentApi.list(1, 1000); // 获取所有学生
      setStudents(response.data.data);
    } catch (error) {
      message.error('获取学生列表失败');
    }
    setLoading(false);
  };

  const handleStudentIdsChange = (value: string) => {
    // 按行分割学号，过滤空行
    const ids = value
      .split('\n')
      .map(line => line.trim())
      .filter(line => line)
      .map(line => parseInt(line))
      .filter(id => !isNaN(id));
    
    setSelectedStudents(ids);
  };

  const handleSelectChange = (value: number[]) => {
    setSelectedStudents(value);
    // 更新文本框
    form.setFieldsValue({
      student_ids_text: value.join('\n')
    });
  };

  const handleSendMessage = async (values: any) => {
    if (selectedStudents.length === 0) {
      message.warning('请选择要发送消息的学生');
      return;
    }

    if (!values.message.trim()) {
      message.warning('请输入要发送的消息');
      return;
    }

    setSendLoading(true);
    try {
      await studentApi.bulkMessage(selectedStudents, values.message);
      message.success(`消息发送成功！共发送给 ${selectedStudents.length} 个学生`);
      form.resetFields();
      setSelectedStudents([]);
    } catch (error) {
      message.error('消息发送失败');
    }
    setSendLoading(false);
  };

  const getSelectedStudentNames = () => {
    return students
      .filter(student => selectedStudents.includes(student.student_id))
      .map(student => ({ name: student.name, student_id: student.student_id }));
  };

  return (
    <div>
      <Card title="群发消息" extra={<UserOutlined />}>
        <Form
          form={form}
          layout="vertical"
          onFinish={handleSendMessage}
        >
          <Form.Item
            name="student_ids_text"
            label="学号列表（按行分割）"
            tooltip="请输入学号，每行一个学号"
          >
            <TextArea
              rows={6}
              placeholder="请输入学号，每行一个，例如：&#10;20210001&#10;20210002&#10;20210003"
              onChange={(e) => handleStudentIdsChange(e.target.value)}
            />
          </Form.Item>

          <Form.Item label="或者通过下拉选择">
            <Select
              mode="multiple"
              placeholder="选择学生"
              style={{ width: '100%' }}
              value={selectedStudents}
              onChange={handleSelectChange}
              loading={loading}
              showSearch
              optionFilterProp="children"
            >
              {students.map((student) => (
                <Option key={student.student_id} value={student.student_id}>
                  {student.name} ({student.student_id})
                </Option>
              ))}
            </Select>
          </Form.Item>

          {selectedStudents.length > 0 && (
            <Form.Item label={`已选择的学生 (${selectedStudents.length}个)`}>
              <div style={{ 
                maxHeight: '150px', 
                overflowY: 'auto', 
                border: '1px solid #d9d9d9', 
                borderRadius: '6px', 
                padding: '8px' 
              }}>
                <Space size={[8, 8]} wrap>
                  {getSelectedStudentNames().map((student) => (
                    <Tag
                      key={student.student_id}
                      closable
                      onClose={() => {
                        const newSelected = selectedStudents.filter(id => id !== student.student_id);
                        setSelectedStudents(newSelected);
                        form.setFieldsValue({
                          student_ids_text: newSelected.join('\n')
                        });
                      }}
                    >
                      {student.name} ({student.student_id})
                    </Tag>
                  ))}
                </Space>
              </div>
            </Form.Item>
          )}

          <Form.Item
            name="message"
            label="消息内容"
            rules={[{ required: true, message: '请输入要发送的消息' }]}
          >
            <TextArea
              rows={4}
              placeholder="请输入要发送的消息内容..."
              showCount
              maxLength={500}
            />
          </Form.Item>

          <Form.Item>
            <Space>
              <Button
                type="primary"
                htmlType="submit"
                icon={<SendOutlined />}
                loading={sendLoading}
                disabled={selectedStudents.length === 0}
              >
                发送消息 ({selectedStudents.length}人)
              </Button>
              <Button onClick={() => {
                form.resetFields();
                setSelectedStudents([]);
              }}>
                清空
              </Button>
            </Space>
          </Form.Item>
        </Form>
      </Card>

      {/* 消息发送历史 */}
      <Card title="发送提示" style={{ marginTop: 16 }}>
        <List size="small">
          <List.Item>
            <span>💡 输入学号时，每行一个学号</span>
          </List.Item>
          <List.Item>
            <span>💡 可以通过学号输入或下拉选择两种方式选择学生</span>
          </List.Item>
          <List.Item>
            <span>💡 消息会发送到学生绑定的QQ号</span>
          </List.Item>
          <List.Item>
            <span>💡 建议消息内容简洁明了，避免过长</span>
          </List.Item>
        </List>
      </Card>
    </div>
  );
};

export default BulkMessage;
