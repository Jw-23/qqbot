import React from 'react';
import { BrowserRouter as Router, Routes, Route, useNavigate, useLocation } from 'react-router-dom';
import { Layout, Menu } from 'antd';
import { UserOutlined, BookOutlined, SettingOutlined, MessageOutlined } from '@ant-design/icons';
import StudentManagement from './pages/StudentManagement';
import GradeManagement from './pages/GradeManagement';
import ConfigManagement from './pages/ConfigManagement';
import BulkMessage from './pages/BulkMessage';
import './App.css';

const { Header, Content, Sider } = Layout;

const AppContent: React.FC = () => {
  const navigate = useNavigate();
  const location = useLocation();
  
  const getSelectedKey = () => {
    switch (location.pathname) {
      case '/students':
        return '1';
      case '/grades':
        return '2';
      case '/bulk-message':
        return '3';
      case '/config':
        return '4';
      default:
        return '1';
    }
  };

  const menuItems = [
    {
      key: '1',
      icon: <UserOutlined />,
      label: '学生管理',
    },
    {
      key: '2',
      icon: <BookOutlined />,
      label: '成绩管理',
    },
    {
      key: '3',
      icon: <MessageOutlined />,
      label: '群发消息',
    },
    {
      key: '4',
      icon: <SettingOutlined />,
      label: '系统配置',
    }
  ];

  const handleMenuClick = ({ key }: { key: string }) => {
    switch (key) {
      case '1':
        navigate('/students');
        break;
      case '2':
        navigate('/grades');
        break;
      case '3':
        navigate('/bulk-message');
        break;
      case '4':
        navigate('/config');
        break;
    }
  };

  return (
    <Layout style={{ minHeight: '100vh' }}>
      <Sider collapsible>
        <div className="logo" style={{ height: 32, margin: 16, background: 'rgba(255, 255, 255, 0.3)' }} />
        <Menu
          theme="dark"
          selectedKeys={[getSelectedKey()]}
          mode="inline"
          items={menuItems}
          onClick={handleMenuClick}
        />
      </Sider>
      <Layout className="site-layout">
        <Header className="site-layout-background" style={{ padding: 0, background: '#fff' }}>
          <h1 style={{ margin: '0 16px', fontSize: '18px' }}>QQ机器人管理后台</h1>
        </Header>
        <Content style={{ margin: '16px' }}>
          <div className="site-layout-background" style={{ padding: 24, minHeight: 360, background: '#fff' }}>
            <Routes>
              <Route path="/" element={<StudentManagement />} />
              <Route path="/students" element={<StudentManagement />} />
              <Route path="/grades" element={<GradeManagement />} />
              <Route path="/bulk-message" element={<BulkMessage />} />
              <Route path="/config" element={<ConfigManagement />} />
            </Routes>
          </div>
        </Content>
      </Layout>
    </Layout>
  );
};

function App() {
  return (
    <Router>
      <AppContent />
    </Router>
  );
}

export default App;
