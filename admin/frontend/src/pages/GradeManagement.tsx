import React, { useState, useEffect } from 'react';
import {
  Table,
  Button,
  Modal,
  Form,
  Input,
  InputNumber,
  Select,
  message,
  Space,
  Popconfirm,
} from 'antd';
import { PlusOutlined, DeleteOutlined, EditOutlined } from '@ant-design/icons';
import { Grade, Student } from '../types';
import { gradeApi, studentApi } from '../services/api';

const { Option } = Select;

const GradeManagement: React.FC = () => {
  const [grades, setGrades] = useState<Grade[]>([]);
  const [students, setStudents] = useState<Student[]>([]);
  const [loading, setLoading] = useState(false);
  const [modalVisible, setModalVisible] = useState(false);
  const [editingGrade, setEditingGrade] = useState<Grade | null>(null);
  const [form] = Form.useForm();
  const [pagination, setPagination] = useState({
    current: 1,
    pageSize: 10,
    total: 0,
  });

  useEffect(() => {
    fetchGrades();
    fetchStudents();
  }, [pagination.current, pagination.pageSize]);

  const fetchGrades = async () => {
    setLoading(true);
    try {
      const response = await gradeApi.list(pagination.current, pagination.pageSize);
      setGrades(response.data.data);
      setPagination(prev => ({
        ...prev,
        total: response.data.total,
      }));
    } catch (error) {
      message.error('获取成绩列表失败');
    }
    setLoading(false);
  };

  const fetchStudents = async () => {
    try {
      const response = await studentApi.list(1, 1000); // 获取所有学生
      setStudents(response.data.data);
    } catch (error) {
      message.error('获取学生列表失败');
    }
  };

  const handleAdd = () => {
    setEditingGrade(null);
    form.resetFields();
    setModalVisible(true);
  };

  const handleEdit = (grade: Grade) => {
    setEditingGrade(grade);
    form.setFieldsValue(grade);
    setModalVisible(true);
  };

  const handleDelete = async (id: number) => {
    try {
      await gradeApi.delete(id);
      message.success('删除成功');
      fetchGrades();
    } catch (error) {
      message.error('删除失败');
    }
  };

  const handleSubmit = async (values: any) => {
    try {
      if (editingGrade) {
        await gradeApi.update(editingGrade.id!, values);
        message.success('更新成功');
      } else {
        await gradeApi.create(values);
        message.success('添加成功');
      }
      setModalVisible(false);
      fetchGrades();
    } catch (error) {
      message.error(editingGrade ? '更新失败' : '添加失败');
    }
  };

  const categories = ['Quiz-1', 'Quiz-2', 'Quiz-3', 'Quiz-4', 'Mid'];

  const columns = [
    {
      title: '学生姓名',
      dataIndex: 'student_name',
      key: 'student_name',
    },
    {
      title: '考试名称',
      dataIndex: 'exam_name',
      key: 'exam_name',
    },
    {
      title: '课程号',
      dataIndex: 'course_id',
      key: 'course_id',
    },
    {
      title: '课序号',
      dataIndex: 'course_seq',
      key: 'course_seq',
    },
    {
      title: '学号',
      dataIndex: 'student_id',
      key: 'student_id',
    },
    {
      title: '成绩',
      dataIndex: 'score',
      key: 'score',
    },
    {
      title: '类别',
      dataIndex: 'category',
      key: 'category',
    },
    {
      title: '操作',
      key: 'action',
      render: (_: any, record: Grade) => (
        <Space size="middle">
          <Button
            type="link"
            icon={<EditOutlined />}
            onClick={() => handleEdit(record)}
          >
            编辑
          </Button>
          <Popconfirm
            title="确定删除这条成绩记录吗？"
            onConfirm={() => handleDelete(record.id!)}
            okText="是"
            cancelText="否"
          >
            <Button type="link" danger icon={<DeleteOutlined />}>
              删除
            </Button>
          </Popconfirm>
        </Space>
      ),
    },
  ];

  return (
    <div>
      <div style={{ marginBottom: 16 }}>
        <Button type="primary" icon={<PlusOutlined />} onClick={handleAdd}>
          添加成绩
        </Button>
      </div>

      <Table
        columns={columns}
        dataSource={grades}
        rowKey="id"
        loading={loading}
        pagination={{
          ...pagination,
          onChange: (page: number, pageSize?: number) => {
            setPagination(prev => ({
              ...prev,
              current: page,
              pageSize: pageSize || 10,
            }));
          },
        }}
      />

      <Modal
        title={editingGrade ? '编辑成绩' : '添加成绩'}
        open={modalVisible}
        onCancel={() => setModalVisible(false)}
        onOk={() => form.submit()}
      >
        <Form
          form={form}
          layout="vertical"
          onFinish={handleSubmit}
        >
          <Form.Item
            name="student_name"
            label="学生姓名"
            rules={[{ required: true, message: '请输入学生姓名' }]}
          >
            <Input />
          </Form.Item>
          <Form.Item
            name="exam_name"
            label="考试名称"
            rules={[{ required: true, message: '请输入考试名称' }]}
          >
            <Input />
          </Form.Item>
          <Form.Item
            name="course_id"
            label="课程号"
            rules={[{ required: true, message: '请输入课程号' }]}
          >
            <InputNumber style={{ width: '100%' }} />
          </Form.Item>
          <Form.Item
            name="course_seq"
            label="课序号"
            rules={[{ required: true, message: '请输入课序号' }]}
          >
            <InputNumber style={{ width: '100%' }} />
          </Form.Item>
          <Form.Item
            name="student_id"
            label="学号"
            rules={[{ required: true, message: '请选择学生' }]}
          >
            <Select
              showSearch
              placeholder="选择学生"
              optionFilterProp="children"
            >
              {students.map((student) => (
                <Option key={student.student_id} value={student.student_id}>
                  {student.name} ({student.student_id})
                </Option>
              ))}
            </Select>
          </Form.Item>
          <Form.Item
            name="score"
            label="成绩"
            rules={[{ required: true, message: '请输入成绩' }]}
          >
            <InputNumber min={0} max={100} style={{ width: '100%' }} />
          </Form.Item>
          <Form.Item
            name="category"
            label="类别"
            rules={[{ required: true, message: '请选择类别' }]}
          >
            <Select placeholder="选择类别">
              {categories.map((category) => (
                <Option key={category} value={category}>
                  {category}
                </Option>
              ))}
            </Select>
          </Form.Item>
        </Form>
      </Modal>
    </div>
  );
};

export default GradeManagement;
