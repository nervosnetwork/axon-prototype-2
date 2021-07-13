export default interface TaskService {
  start(): Promise<void>;

  wrapperedTask(): Promise<void>;

  task(): Promise<void>;
}
