import { evaluateFormAction } from '../../../rhwp-shared/forms/action-runtime.ts';

self.onmessage = ({ data }) => {
  try { self.postMessage({ result: evaluateFormAction(data) }); }
  catch (error) {
    // 실행기가 만든 FORM_ACTION_SCRIPT 사유는 그대로 알리고, 그 밖의 내부 오류는 일반 문구로 바꾼다.
    const reason = error instanceof Error && error.message.startsWith('FORM_ACTION_SCRIPT: ') ? error.message : '';
    self.postMessage({ error: reason || 'FORM_ACTION_SCRIPT: 처리할 수 없는 명령이거나 실행 한도를 초과했습니다.' });
  }
};
