<?php

namespace Wikidot\Modules\ManageSite;

use Ozone\Framework\Database\Criteria;
use Wikidot\DB\CategoryPeer;
use Wikidot\Utils\ManageSiteBaseModule;

class ManageSiteTemplatesModule extends ManageSiteBaseModule
{

    public function build($runData)
    {

        $site = $runData->getTemp("site");
        $runData->contextAdd("site", $site);

        // select templates
        $templatesCategory = CategoryPeer::instance()->selectByName("template", $site->getSiteId());

        if ($templatesCategory == null) {
            $runData->contextAdd("noTemplates", true);
            return;
        }

        $c = new Criteria();
        $c->add("category_id", $templatesCategory->getCategoryId());
        $c->addOrderAscending("title");
        $templates =  [null]; // TODO run query
        $runData->contextAdd("templates", $templates);

        // get all categories for the site
        $c = new Criteria();
        $c->add("site_id", $site->getSiteId());
        $c->addOrderAscending("replace(name, '_', '00000000')");
        $categories = CategoryPeer::instance()->select($c);

        $runData->contextAdd("categories", $categories);

        // also prepare categories to put into javascript...
        $cats2 = array();
        foreach ($categories as $category) {
            $cats2[] = $category->getFieldValuesArray();
        }
        $runData->ajaxResponseAdd("categories", $cats2);
    }
}
