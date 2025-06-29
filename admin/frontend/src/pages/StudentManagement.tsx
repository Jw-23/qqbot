import React, { useState, useEffect } from 'react';
import {
  Table,
  Button,
  Modal,
  Form,
  Input,
  InputNumber,
  message,
  Space,
  Upload,
  Popconfirm,
} from 'antd';
import { PlusOutlined, UploadOutlined, DownloadOutlined, DeleteOutlined, EditOutlined } from '@ant-design/icons';
import { Student } from '../types';
import { studentApi } from '../services/api';

const StudentManagement: React.FC = () => {
  const [students, setStudents] = useState<Student[]>([]);
  const [loading, setLoading] = useState(false);
  const [modalVisible, setModalVisible] = useState(false);
  const [editingStudent, setEditingStudent] = useState<Student | null>(null);
  const [form] = Form.useForm();
  const [pagination, setPagination] = useState({
    current: 1,
    pageSize: 10,
    total: 0,
  });

  useEffect(() => {
    fetchStudents();
  }, [pagination.current, pagination.pageSize]);

  const fetchStudents = async () => {
    setLoading(true);
    try {
      const response = await studentApi.list(pagination.current, pagination.pageSize);
      setStudents(response.data.data);
      setPagination(prev => ({
        ...prev,
        total: response.data.total,
      }));
    } catch (error) {
      message.error('获取学生列表失败');
    }
    setLoading(false);
  };

  const handleAdd = () => {
    setEditingStudent(null);
    form.resetFields();
    setModalVisible(true);
  };

  const handleEdit = (student: Student) => {
    setEditingStudent(student);
    form.setFieldsValue(student);
    setModalVisible(true);
  };

  const handleDelete = async (id: number) => {
    try {
      await studentApi.delete(id);
      message.success('删除成功');
      fetchStudents();
    } catch (error) {
      message.error('删除失败');
    }
  };

  const handleSubmit = async (values: any) => {
    try {
      if (editingStudent) {
        await studentApi.update(editingStudent.id!, values);
        message.success('更新成功');
      } else {
        await studentApi.create(values);
        message.success('添加成功');
      }
      setModalVisible(false);
      fetchStudents();
    } catch (error) {
      message.error(editingStudent ? '更新失败' : '添加失败');
    }
  };

  const handleExport = async () => {
    try {
      const response = await studentApi.export();
      const url = window.URL.createObjectURL(new Blob([response.data]));
      const link = document.createElement('a');
      link.href = url;
      link.setAttribute('download', 'students.csv');
      document.body.appendChild(link);
      link.click();
      link.remove();
      window.URL.revokeObjectURL(url);
    } catch (error) {
      message.error('导出失败');
    }
  };

  const handleImport = async (file: File) => {
    const text = await file.text();
    const lines = text.split('\n').filter(line => line.trim());
    const students = lines.slice(1).map(line => {
      const [student_id, name, qq_number, group_id] = line.split(',');
      return {
        student_id: parseInt(student_id),
        name: name.trim(),
        qq_number: parseInt(qq_number),
        group_id: parseInt(group_id),
      };
    });

    try {
      await studentApi.import(students);
      message.success('导入成功');
      fetchStudents();
    } catch (error) {
      message.error('导入失败');
    }
    return false;
  };

  const columns = [
    {
      title: '学号',
      dataIndex: 'student_id',
      key: 'student_id',
    },
    {
      title: '姓名',
      dataIndex: 'name',
      key: 'name',
    },
    {
      title: 'QQ号',
      dataIndex: 'qq_number',
      key: 'qq_number',
    },
    {
      title: '群号',
      dataIndex: 'group_id',
      key: 'group_id',
    },
    {
      title: '创建时间',
      dataIndex: 'created_at',
      key: 'created_at',
      render: (text: string) => text ? new Date(text).toLocaleString() : '',
    },
    {
      title: '操作',
      key: 'action',
      render: (_: any, record: Student) => (
        <Space size="middle">
          <Button
            type="link"
            icon={<EditOutlined />}
            onClick={() => handleEdit(record)}
          >
            编辑
          </Button>
          <Popconfirm
            title="确定删除这个学生吗？"
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
        <Space>
          <Button type="primary" icon={<PlusOutlined />} onClick={handleAdd}>
            添加学生
          </Button>
          <Upload
            accept=".csv"
            beforeUpload={handleImport}
            showUploadList={false}
          >
            <Button icon={<UploadOutlined />}>导入CSV</Button>
          </Upload>
          <Button icon={<DownloadOutlined />} onClick={handleExport}>
            导出CSV
          </Button>
        </Space>
      </div>

      <Table
        columns={columns}
        dataSource={students}
        rowKey="id"
        loading={loading}
        pagination={{
          ...pagination,
          onChange: (page, pageSize) => {
            setPagination(prev => ({
              ...prev,
              current: page,
              pageSize: pageSize || 10,
            }));
          },
        }}
      />

      <Modal
        title={editingStudent ? '编辑学生' : '添加学生'}
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
            name="student_id"
            label="学号"
            rules={[{ required: true, message: '请输入学号' }]}
          >
            <InputNumber style={{ width: '100%' }} />
          </Form.Item>
          <Form.Item
            name="name"
            label="姓名"
            rules={[{ required: true, message: '请输入姓名' }]}
          >
            <Input />
          </Form.Item>
          <Form.Item
            name="qq_number"
            label="QQ号"
            rules={[{ required: true, message: '请输入QQ号' }]}
          >
            <InputNumber style={{ width: '100%' }} />
          </Form.Item>
          <Form.Item
            name="group_id"
            label="群号"
            rules={[{ required: true, message: '请输入群号' }]}
          >
            <InputNumber style={{ width: '100%' }} />
          </Form.Item>
        </Form>
      </Modal>
    </div>
  );
};

export default StudentManagement;
