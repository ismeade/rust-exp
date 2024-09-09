use k8s_openapi::api::apps::v1::Deployment;
use k8s_openapi::api::core::v1::Pod;
use kube::{api::{Api, ListParams, Patch, PatchParams}, Client};
use kube::api::GetParams;
use serde_json::json;

#[tokio::main(flavor = "current_thread")]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Infer the runtime environment and try to create a Kubernetes Client
    let client = Client::try_default().await?;

    get_deployment_from_selector(client.clone()).await;

    get_deployment_from_name(client.clone()).await;

    update_deployment_image(client.clone()).await;

    Ok(())
}

async fn get_deployment_from_selector(client: Client) {
    // Read pods in the configured namespace into the typed interface from k8s-openapi
    let api: Api<Deployment> = Api::namespaced(client.clone(), "test");

    // 定义查询参数
    let selector = Some("metadata.name=nginx-1".to_string());
    let lp = ListParams {
        field_selector: selector,
        label_selector: None,
        timeout: None,
        limit: None,
        continue_token: None,
        version_match: None,
        resource_version: None,
    };


    let result = api.list(&lp).await;

    match result {
        Ok(result) => {
            let mut count = 0;

            for deployment in result {
                // for deployment in deployments.list(&ListParams::default()).await? {
                // println!("found pod {}", p.metadata.name.unwrap());
                // let mut spec = deployment.spec.as_mut().unwrap().template;
                // let mut template_spec = spec.spec.unwrap();
                // let mut containers = template_spec.containers;
                // let mut container = containers.first_mut().unwrap();

                count = count + 1;

                println!("image {}", deployment.spec.unwrap().template.spec.as_mut().unwrap().containers.first_mut().unwrap().image.as_mut().unwrap());

                // println!("found pod {}", deployment.name_any());
            }

            println!("count: {}", count);

        },
        _Err => println!("查询出错")
    }

}

// #[tokio::main(flavor = "current_thread")]
async fn get_deployment_from_name(client: Client) {

    let deployments: Api<Deployment> = Api::namespaced(client, "test");

    let d_name = "nginx-11";

    match deployments.get(d_name).await {
        Ok(deployment) => {
            match deployment.metadata.resource_version {
                Some(rv) => {
                    println!("rv: {}", rv);
                },
                None => println!("没找到resource_version")
            }
            println!("image {}", deployment.spec.unwrap().template.spec.as_mut().unwrap().containers.first_mut().unwrap().image.as_mut().unwrap());
        }
        _Err => {
            println!("没找到: {}", d_name);
        }
    }

}

// #[tokio::main(flavor = "current_thread")]
async fn update_deployment_image(client: Client) {
    // Deployment 名称
    let deployment_name = "nginx-1";

    // 新镜像名称
    let new_image = "nginx:alpine";

    // 构造 Patch 文档
    let patch = json!({
        "spec": {
            "template": {
                "spec": {
                    "containers": [
                        {
                            "name": "nginx-1",
                            "image": new_image
                        }
                    ]
                }
            }
        }
    });

    // 更新 Deployment
    let patch_params = PatchParams::apply("your-application-name"); // 选择适当的策略名称

    let api: Api<Deployment> = Api::namespaced(client.clone(), "test");
    let result = api.patch(deployment_name, &patch_params, &Patch::Strategic(&patch)).await;
    match result {
        Ok(result) => println!("Deployment updated: {:?}", result),
        _Err => println!("更新失败")
    }

}