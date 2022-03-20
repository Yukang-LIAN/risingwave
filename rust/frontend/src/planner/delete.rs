// Copyright 2022 Singularity Data
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
// http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.
//
use fixedbitset::FixedBitSet;
use risingwave_common::error::Result;

use super::Planner;
use crate::binder::BoundDelete;
use crate::optimizer::plan_node::{LogicalDelete, LogicalFilter};
use crate::optimizer::property::{Distribution, Order};
use crate::optimizer::{PlanRef, PlanRoot};

impl Planner {
    pub(super) fn plan_delete(&mut self, delete: BoundDelete) -> Result<PlanRoot> {
        let scan = self.plan_base_table_ref(delete.table.clone())?;
        let input = if let Some(expr) = delete.selection {
            LogicalFilter::create(scan, expr)?
        } else {
            scan
        };
        let plan: PlanRef = LogicalDelete::create(input, delete.table)?.into();

        let order = Order::any().clone();
        let dist = Distribution::Single;
        let mut out_fields = FixedBitSet::with_capacity(plan.schema().len());
        out_fields.insert_range(..);

        let root = PlanRoot::new(plan, dist, order, out_fields);
        Ok(root)
    }
}