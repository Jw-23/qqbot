export interface Student {
  id?: number;
  student_id: number;
  name: string;
  qq_number: number;
  group_id: number;
  created_at?: string;
  updated_at?: string;
}

export interface Grade {
  id?: number;
  student_name: string;
  exam_name: string;
  course_id: number;
  course_seq: number;
  student_id: number;
  score: number;
  category: string;
}

export interface Config {
  logging_level: string;
  cmd_suffix: string;
  admins: string[];
  cache: {
    cache_lifetime: string;
    cache_capacity: number;
    cache_idletime: string;
    conversation_capacity: number;
    max_conversation_history: number;
    conversation_timeout_minutes: number;
  };
  database: {
    url: string;
    max_connections: number;
    connect_timeout: string;
    acquire_timeout: string;
    idle_timeout: string;
    max_lifetime: string;
    sqlx_logging: boolean;
  };
  llm: {
    api_key: string;
    base_url: string;
    model: string;
    system_prompt: string;
    temperature: number;
    max_tokens: number;
    top_p: number;
    timeout_seconds: number;
    auto_capture_group_messages: boolean;
  };
}

export interface ApiResponse<T> {
  data: T[];
  total: number;
  page: number;
  limit: number;
}
