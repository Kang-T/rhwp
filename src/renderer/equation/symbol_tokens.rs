//! ASCII 관계 기호의 세 파서 경로를 같은 수학 기호로 정규화한다.

use super::ast::EqNode;
use super::symbols::lookup_symbol;

pub(super) fn symbol_node(value: String) -> EqNode {
    if value == "->" {
        EqNode::MathSymbol("→".to_string())
    } else if let Some(symbol) = lookup_symbol(&value) {
        EqNode::MathSymbol(symbol.to_string())
    } else {
        EqNode::Symbol(value)
    }
}
