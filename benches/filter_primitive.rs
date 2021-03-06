use std::rc::Rc;

#[macro_use]
extern crate criterion;

use criterion::Criterion;

extern crate datafusion;
use datafusion::arrow::*;
use datafusion::exec::*;
use datafusion::rel::*;

extern crate serde_json;

fn criterion_benchmark(c: &mut Criterion) {

    c.bench_function("filter_primitive", move |b| {

        // create execution context
        let ctx = ExecutionContext::local("test/data".to_string());

        // define schema for data source (csv file)
        let schema = Schema::new(vec![
            Field::new("city", DataType::Utf8, false),
            Field::new("lat", DataType::Float64, false),
            Field::new("lng", DataType::Float64, false)]);

        // generate some random data
        let n = 1000;
        let batch : Box<Batch> = Box::new(ColumnBatch { columns: vec![
            Rc::new(Array::new(ArrayData::Utf8((0 .. n).map(|_| "city_name".to_string()).collect()))),
            Rc::new(Array::new(ArrayData::Float64((0 .. n).map(|_| 50.0).collect()))),
            Rc::new(Array::new(ArrayData::Float64((0 .. n).map(|_| 0.0).collect())))
        ]});

        //let lat = df1.col("lat").unwrap();
        let filter_expr = Expr::BinaryExpr {
            left: Box::new(Expr::Column(1)),
            op: Operator::Gt,
            right: Box::new(Expr::Literal(ScalarValue::Float64(52.0)))
        };

        let ctx = ExecutionContext::local("test/data".to_string());
        let compiled_filter_expr = &compile_expr(&ctx, &filter_expr).unwrap();

        let batch_ref: &Box<Batch> = &batch;

        let col_count = batch.col_count();

        b.iter(move || {

            // evaluate the filter expression against every row
            let filter_eval: Rc<Array> = (compiled_filter_expr)(batch_ref.as_ref());

            // filter the columns
            let filtered_columns: Vec<Rc<Array>> = (0..col_count)
                .map(|column_index| { Rc::new(batch_ref.column(column_index).filter(&filter_eval)) })
                .collect();

            // create the new batch with the filtered columns
            let filtered_batch: Box<Batch> = Box::new(ColumnBatch { columns: filtered_columns });
        })
    });

}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
