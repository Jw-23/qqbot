import axios from 'axios';
import { Student, Grade, Config, ApiResponse } from '../types';

const API_BASE_URL = 'http://localhost:8080/api';

const api = axios.create({
  baseURL: API_BASE_URL,
  headers: {
    'Content-Type': 'application/json',
  },
});

// 学生相关API
export const studentApi = {
  list: (page: number = 1, limit: number = 10) =>
    api.get<ApiResponse<Student>>(`/students?page=${page}&limit=${limit}`),
  
  get: (id: number) =>
    api.get<Student>(`/students/${id}`),
  
  create: (student: Omit<Student, 'id' | 'created_at' | 'updated_at'>) =>
    api.post<Student>('/students', student),
  
  update: (id: number, student: Partial<Student>) =>
    api.put<Student>(`/students/${id}`, student),
  
  delete: (id: number) =>
    api.delete(`/students/${id}`),
  
  import: (students: Omit<Student, 'id' | 'created_at' | 'updated_at'>[]) =>
    api.post('/students/import', { students }),
  
  export: () =>
    api.get('/students/export', { responseType: 'blob' }),
  
  bulkMessage: (student_ids: number[], message: string) =>
    api.post('/students/bulk-message', { student_ids, message }),
};

// 成绩相关API
export const gradeApi = {
  list: (page: number = 1, limit: number = 10) =>
    api.get<ApiResponse<Grade>>(`/grades?page=${page}&limit=${limit}`),
  
  get: (id: number) =>
    api.get<Grade>(`/grades/${id}`),
  
  create: (grade: Omit<Grade, 'id'>) =>
    api.post<Grade>('/grades', grade),
  
  update: (id: number, grade: Partial<Grade>) =>
    api.put<Grade>(`/grades/${id}`, grade),
  
  delete: (id: number) =>
    api.delete(`/grades/${id}`),
  
  getByStudent: (student_id: number) =>
    api.get<Grade[]>(`/grades/student/${student_id}`),
};

// 配置相关API
export const configApi = {
  get: () =>
    api.get<Config>('/config'),
  
  update: (config: Config) =>
    api.put<string>('/config', config),
};
