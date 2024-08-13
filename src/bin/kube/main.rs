use k8s_openapi::api::apps::v1::Deployment;
use kube::{api::{Api, ListParams, Patch, PatchParams}, Client};
use serde_json::json;

#[tokio::main(flavor = "current_thread")]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Infer the runtime environment and try to create a Kubernetes Client
    let client = Client::try_default().await?;

    // Read pods in the configured namespace into the typed interface from k8s-openapi
    let deployments: Api<Deployment> = Api::namespaced(client, "test");

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
    for deployment in deployments.list(&lp).await? {
        // for deployment in deployments.list(&ListParams::default()).await? {
        // println!("found pod {}", p.metadata.name.unwrap());
        // let mut spec = deployment.spec.as_mut().unwrap().template;
        // let mut template_spec = spec.spec.unwrap();
        // let mut containers = template_spec.containers;
        // let mut container = containers.first_mut().unwrap();

        println!("image {}", deployment.spec.unwrap().template.spec.as_mut().unwrap().containers.first_mut().unwrap().image.as_mut().unwrap());


        // println!("found pod {}", deployment.name_any());
    }

    // 直接用名字获取
    let nginx = deployments.get("nginx-1").await?;
    println!("image {}", nginx.spec.unwrap().template.spec.as_mut().unwrap().containers.first_mut().unwrap().image.as_mut().unwrap());

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
    let result = deployments.patch(deployment_name, &patch_params, &Patch::Strategic(&patch)).await?;

    println!("Deployment updated: {:?}", result);


    Ok(())
}
